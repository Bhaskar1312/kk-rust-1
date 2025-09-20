fn main() {
    let s1 = String::from("hello");
    let len1 = calculate_length_ownership(s1);
    // println!("The length of {} is {}.", s1, len1);
    // println!("{}", len1);

    let s2 = String::from("world");
    let len2 = calculate_length_borrowing(&s2);
    println!("The length of {} is {}.", s2, len2);
}
fn calculate_length_ownership(s: String) -> usize {
    s.len()
}

fn calculate_length_borrowing(s: &String) -> usize {
    s.len()
}
