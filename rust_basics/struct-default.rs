// Your struct definition here

// You implementation of the trait here
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Default for Rectangle {
    fn default() -> Self {
        Rectangle {
            width: 10,
            height: 20,
        }
    }
}


fn main() {
    let rect: Rectangle = Default::default();
    println!("{:?}", rect);
}
