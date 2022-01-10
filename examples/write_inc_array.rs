use std::env;
use std::fs::create_dir_all;
use std::path::PathBuf;
use npy_stream::NPYStream;

fn main() {

    let args : Vec<String> = env::args().collect();

    let num_rows = args.get(1)
        .unwrap_or_else(|| panic!("First arg should be number of rows to write"))
        .parse::<usize>().unwrap();

    let num_cols = args.get(2)
        .unwrap_or_else(|| panic!("Second arg should be number of cols to write"))
        .parse::<usize>().unwrap();

    let output_dir : PathBuf = [env!("CARGO_MANIFEST_DIR"), "example_output"].iter().collect();
    create_dir_all(&output_dir).unwrap();
    
    let output_filename = output_dir.join(format!("inc_array_{}_{}.npy", num_rows, num_cols));

    let mut npy_stream = NPYStream::new(&output_filename, num_cols);
    
    let mut v : Vec<f32> = (0..num_cols).map(|x| x as f32).collect();

    for _ in 0..num_rows {
        npy_stream.write(&v);

        for x in v.iter_mut() {
            *x += num_cols as f32;
        }
    }

}
