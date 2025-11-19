// Your trait here
trait Greet {
    fn hello(&self) -> String {
        String::from("Hello, world!")
    }
}
struct Human;

// Your trait implementation here
impl Greet for Human {
    fn hello(&self) -> String {
        String::from("Hello, Human!")
    }
}

// Do not modify
fn main() {
    let person = Human;
    println!("{}", person.hello());
}
