use std::collections::HashMap;

fn main() {
    // Your hashmap here
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // Your match block here
    match score {
        Some(&x) => println!("Score: {}", x),
        None => println!("Team not found"),
    }
}
