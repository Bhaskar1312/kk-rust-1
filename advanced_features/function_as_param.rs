fn number_to_string(num: &i32) -> String {
    format!("Number: {}", num)
}
fn main() {
    let numbers = vec![1, 2, 3];
    let strings: Vec<String> = numbers.iter()
        .map(|num| format!("Number: {}", num))
        .collect();
    println!("{:?}", strings);

    // let strings: Vec<String> = numbers.iter()
    //     .map(|num| number_to_string(num))
    //     .collect();
    // println!("{:?}", strings);
}