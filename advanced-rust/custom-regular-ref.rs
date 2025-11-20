use std::ops::Deref;
// Your tuple struct here
struct MySmartPointer<T>(T);

// Your new function constructor implementation for struct here
impl <T> MySmartPointer<T> {
    fn new(x: T) -> MySmartPointer<T> {
        MySmartPointer(x)
    }
}
// Your Deref trait implementation here
impl <T> Deref for MySmartPointer<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

// Do not modify
fn main() {
    let x = 5;
    let y = MySmartPointer::new(x);
    assert_eq!(5, *y);
}
