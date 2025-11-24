use std::path::Path;

fn main() {
    let path = Path::new("data.txt");
    if path.exists() {
        println!("File exists.");
    } else {
        println!("File does not exist.");
    }
}
// cargo run --bin check-file