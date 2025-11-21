fn main() {
    let mut count = 0;
    let mut increment = || {
        count+=1;
        count
    };
    println!("First call: {}", increment());
    println!("Second call: {}", increment());
}