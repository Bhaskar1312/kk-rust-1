// Complete this function
fn find_max<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }
    return Some(list.iter().max_by(|a, b| a.partial_cmp(b).expect("REASON")).unwrap());
}

fn main() {
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    if let Some(max) = find_max(&numbers) {
        println!("The maximum value is: {}", max);
    } else {
        println!("The list is empty.");
    }
    
    let words = vec!["apple", "banana", "pear", "orange"];
    if let Some(max) = find_max(&words) {
        println!("The maximum word is: {}", max);
    }
}
