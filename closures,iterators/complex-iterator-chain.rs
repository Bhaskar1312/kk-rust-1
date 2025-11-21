fn main() {
    let numbers = vec![2, 5, 10, 12, 15];
    let result: i32 = numbers.iter()
        .filter(|&&x| x >= 10)
        .map(|&x| x*x)
        .sum();
    println!("Sum of squares of numbers >= 10: {}", result);

}