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

fn make_header(num_cols: usize) -> String {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
