
struct MySmartPointer {
    data: String,
}

impl Drop for MySmartPointer {
    fn drop(&mut self) {
        println!("Dropping MySmartPointer with data `{}`!", self.data);
    }
}
fn main() {
    let a = MySmartPointer { data: String::from("my data") };
    println!("MySmartPointer created.");
    drop(a);
    println!("MySmartPointer dropped before the end of main.");
}