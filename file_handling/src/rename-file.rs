use std::fs;

fn main() {
    fs::rename("old_name.txt", "new_name.txt").expect("Unable to rename file");
}