use std::time::Instant;

fn main() {
    let large_string = "a".repeat(10000000);

    let start = Instant::now();
    let _cloned_string = large_string.clone();
    let duration = start.elapsed();
    println!("Time taken to clone: {:?}", duration);

    let start = Instant::now();
    let _moved_string = large_string;
    let duration = start.elapsed();
    println!("Time taken to move: {:?}", duration);

}