fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let processed: Vec<i32> = numbers.iter()
                        .filter(|&&x| x % 2 != 0)
                        .map(|&x| x*2)
                        .collect();
    println!("Processed numbers: {:?}", processed);
}