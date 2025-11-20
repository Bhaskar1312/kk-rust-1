// Your struct here
struct CustomSmartPointer {
    data: String,
}
// Your Drop implementation here
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
// Do not modify
fn main() {
    let c = CustomSmartPointer {
        data: String::from("my data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
