// Your function here
fn last_element<T>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        None
    } else {
        Some(&list[list.len() - 1])
    }
}
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let empty: Vec<i32> = Vec::new();
    
    // Calling last_element with a non-empty list
    if let Some(last) = last_element(&numbers) {
        println!("The last element of numbers is: {}", last);
    } else {
        println!("The list numbers is empty.");
    }
    
    // Calling last_element with an empty list
    if let Some(last) = last_element(&empty) {
        println!("The last element of empty is: {}", last);
    } else {
        println!("The list empty is empty.");
    }
}
