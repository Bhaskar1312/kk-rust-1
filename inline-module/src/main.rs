mod calculator;
use calculator::{add, subtract};
fn main() {
    let sum = add(5, 3);
    let diff = subtract(5, 3);
    println!("Sum: {}, Difference: {}", sum, diff);
}
