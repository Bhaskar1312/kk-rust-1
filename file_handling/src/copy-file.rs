use std::fs;

fn main() {
    fs::copy("source.txt", "destination.txt").expect("Unable to copy file");
}