use std::path::Path;
use std::fs::File;
use std::io::{BufWriter, Write, Seek};

pub struct NPYStream {
    writer: BufWriter<File>,
    num_cols: usize,
    num_rows: usize,
}

impl NPYStream {
    pub fn new(path: &Path, num_cols: usize) -> Self {
        let file = File::create(path).unwrap();   // FIXME - error handling
        let mut writer = BufWriter::new(file);

        // Write the header to the file.
        // This will be overwritten when the struct is finally dropped, but writing this now
        // ensures that the correct number of bytes at the start of the file are reserved for the
        // header.
        writer.write(&make_header(0, num_cols))
            .unwrap_or_else(|err| panic!("Error writing header: {:?}", err));

        NPYStream {writer, num_cols, num_rows: 0}
    }

    pub fn write(&mut self, row: &Vec<f32>) {
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
        self.writer.flush()
            .and_then(|()| self.writer.rewind())
            .and_then(|()| self.writer.write(&make_header(self.num_rows, self.num_cols)))
            .and_then(|_num_bytes_written| self.writer.flush())
            .unwrap_or_else(|err| panic!("Error closing numpy stream: {:?}", err));
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
    use std::{io::{BufReader, Read}, str::from_utf8};
    use crate::make_header;
    use tempfile::tempdir;
    use regex::Regex;
    use super::*;

    #[test]
    fn header_len_correct() {
        assert_eq!(make_header(1, 1).len(), 128);
        assert_eq!(make_header(100, 100).len(), 128);
        assert_eq!(make_header(1_000_000, 100).len(), 128);
    }

    #[test]
    fn small_write_succeeds() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("small_array.npy");

        let mut npy_stream : NPYStream = NPYStream::new(&file_path, 2);

        npy_stream.write(&vec![0.0, 1.0]);
        npy_stream.write(&vec![2.0, 3.0]);

        drop(npy_stream);   // Cause the file to be closed

        // Re-open the file and verify some properties of the write.
        let file = File::open(file_path).unwrap();
        let mut reader = BufReader::new(file);

        let mut bytes : Vec<u8> = vec![];
        reader.read_to_end(&mut bytes).unwrap();

        // Check size matches that of a header, plus 4 floats (each 4 bytes).
        assert_eq!(bytes.len(), 128 + 4*4);

        // Drop the non-string (magic number, version, and size) parts, and use the remained of the
        // header to check that the size was written correctly.
        let header = from_utf8(&bytes[10..128]).unwrap();

        let re = Regex::new(r"'shape': \(([0-9]+), ([0-9]+)\)").unwrap();
        assert!(re.is_match(header));

        let cap = re.captures(header).unwrap();
        let rows_actual = cap[1].parse::<usize>().unwrap();
        let cols_actual = cap[2].parse::<usize>().unwrap();

        // Check that we actualy did overwrite the header at the end.
        assert_eq!(rows_actual, 2);
        assert_eq!(cols_actual, 2);
    }
}
