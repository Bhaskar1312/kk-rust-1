fn main() {
    let pair = (1, 2);
    match pair {
        (x, y) if x == y => println!("Both values are equal: {}", x),
        (x, y) if x + y = 0 => println!("Sum is zero: {}", x + y),
        _ => println!("Values are different or sum is not zero"),
    }
}