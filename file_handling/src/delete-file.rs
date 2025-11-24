use std::fs;

fn main() {
    if fs::remove_file("temp.txt").is_ok() {
        println!("File deleted successfully.");
    } else {
        println!("File does not exist or could not be deleted.");
    }
}