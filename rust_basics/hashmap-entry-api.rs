use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // Your entry API methods here
    scores.entry(String::from("Blue")).and_modify(|e| *e += 50).or_insert(50);
    // scores.insert(String::from("Green"), 50);
    scores.entry(String::from("Green")).or_insert(50);
    
    println!("{:?}", scores);
}
