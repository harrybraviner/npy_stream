use std::fs::File;

pub struct NPYStream {
    file: File,
    num_cols: usize,
    num_rows: usize,
}

impl NPYStream {
    pub fn new(num_cols: usize) -> Self {
        unimplemented!()
    }

    pub fn write(&mut self, row: Vec<f32>) {
        assert_eq!(self.num_cols, row.len());

        unimplemented!()
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
