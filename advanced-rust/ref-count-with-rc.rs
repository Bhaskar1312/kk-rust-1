use std::rc::Rc;

fn main() {
    let shared_data = Rc::new(String::from("shared data"));
    println!("Reference count: {}", Rc::strong_count(&shared_data));
    let a = Rc::clone(&shared_data);
    println!("Reference count after creating `a`: {}", Rc::strong_count(&a));
    
    let b = Rc::clone(&shared_data);
    println!("Reference count after creating `b`: {}", Rc::strong_count(&b));
    
}