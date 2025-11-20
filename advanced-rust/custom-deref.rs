use std::ops::Deref;
// Your struct here
struct MyBox<T>(T);
// Your new function implementation for the struct here
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Your Deref trait implementation here
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

// Do not modify
fn main() {
    let x = 5;
    let y = MyBox::new(x);
    println!("Value inside MyBox: {}", *y);
}
