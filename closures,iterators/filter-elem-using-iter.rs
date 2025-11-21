fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let even_numbers: Vec<i32> = numbers.iter().filter(|&&x| x%2==0).cloned().collect();
    println!("Even numbers: {:?}", even_numbers);
}