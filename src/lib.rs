use std::path::Path;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct NPYStream {
    writer: BufWriter<File>,
    num_cols: usize,
    num_rows: usize,
}

impl NPYStream {
    pub fn new(path: &Path, num_cols: usize) -> Self {
        let file = File::open(path).unwrap();   // FIXME - error handling
        let mut writer = BufWriter::new(file);

        // Write the header to the file.
        // This will be overwritten when the struct is finally dropped, but writing this now
        // ensures that the correct number of bytes at the start of the file are reserved for the
        // header.
        writer.write(&make_header(0, num_cols));

        NPYStream {writer, num_cols, num_rows: 0}
    }

    pub fn write(&mut self, row: Vec<f32>) {
        assert_eq!(self.num_cols, row.len());

        for x in row {
            let bytes = x.to_le_bytes();
            self.writer.write(&bytes).unwrap_or_else(|err|
                panic!("Error writing values on row {}: {:?}", self.num_rows, err));
        }

        self.num_rows += 1;
    }

}

impl Drop for NPYStream {
    fn drop(&mut self) {
        // FIXME - write number of rows to shape
        unimplemented!()
    }
}

fn make_header(num_rows: usize, num_cols: usize) -> Vec<u8> {
    let header_prefix : &[u8] = b"\x93NUMPY\x01\x00";   // Format version 1.0

    let dtype_str = "<f4";  // Little-endian, 4 byte (32 bit) float
    let header_str = format!("{{'descr': '{}', 'fortran_order': False, 'shape': ({}, {})}}",
        dtype_str, num_rows, num_cols);
    let header = header_str.as_bytes();
    
    let header_size_bytes = (header.len() as u16).to_le_bytes();

    let padding_required = 128 - (header_prefix.len() + header_size_bytes.len() + header.len() + 1);

    // Concat, pad, and add newline.
    let padding = vec![b' '; padding_required];

    let full_header = [
        header_prefix,
        &header_size_bytes,
        header,
        &padding,
        b"\n"
    ].concat();

    assert_eq!(full_header.len(), 128);

    return full_header;
}

#[cfg(test)]
mod tests {
    use crate::make_header;

    #[test]
    fn header_len_correct() {
        assert_eq!(make_header(1, 1).len(), 128);
        assert_eq!(make_header(100, 100).len(), 128);
        assert_eq!(make_header(1_000_000, 100).len(), 128);
    }
}
