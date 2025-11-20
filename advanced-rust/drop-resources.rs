// Your struct here
struct Resource {
    name: String,
}
// Your Drop implementation here
impl Drop for Resource {
    fn drop(&mut self) {
        println!("Releasing resource: {}", self.name);
    }
}

// Do not modify
fn main() {
    let resource = Resource {
        name: String::from("Network Connection"),
    };
    println!("Resource acquired.");
}
