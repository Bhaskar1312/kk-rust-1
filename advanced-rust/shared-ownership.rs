use std::rc::Rc;

fn main() {
    let shared_data = Rc::new(String::from("shared data"));
    let a = Rc::clone(&shared_data);
    let b = Rc::clone(&shared_data);
    println!("a: {}, b: {}, shared_data: {}", a, b, shared_data);
}