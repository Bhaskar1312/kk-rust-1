use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::create("output.txt").expect("Unable to create file");
    file.write_all(b"Hello, Rust!").expect("Unable to write data");

}
// cargo run --bin create-write-to-file