// Your struct here
struct Rectangle {
    width: u32,
    height: u32,
}
// Your method implementation here
impl Rectangle {
    fn increase_width(&mut self, increment: u32) {
        self.width += increment;
    }
}
fn main() {
    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };
    rect.increase_width(10);
    println!("Updated width: {}", rect.width);
}
