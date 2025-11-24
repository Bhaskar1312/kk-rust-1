use std::fs;

fn main() {
    fs::create_dir("my_directory").expect("Unable to create directory");
    println!("Directory created successfully.");
}
