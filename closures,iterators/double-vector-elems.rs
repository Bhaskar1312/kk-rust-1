fn main() {
    let numbers = vec![1, 2, 3, 4];
    let doubled: Vec<i32> = numbers.iter().map(|x| x*2).collect();
    println!("Doubled numbers: {:?}", doubled);
}