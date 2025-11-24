use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new()
        .append(true)
        .open("log.txt")
        .expect("Unable to open file");
    file.write_all(b"Welcome to Rust programming!")
        .expect("Unable to write data");
}
// cargo run --bin append-to-file