use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read data");
    println!("File contents: {}", contents);
}
// cargo run --bin read-from-file