// Your trait here
trait Container {
    type Item;
    fn new(item: Self::Item) -> Self;
    fn get(&self) -> &Self::Item;
}
struct BoxedItem<T> {
    item: T,
}

// Your trait implementation for BoxedItem here
impl<T> Container for BoxedItem<T> {
    type Item = T;

    fn new(item: T) -> Self {
        BoxedItem {item}
    }
    fn get(&self) -> &T {
        &self.item
    }
}

struct BaggedItem<T> {
    item: T,
}

// Your trait implementation for BaggedItem here
impl<T> Container for BaggedItem<T> {
    type Item = T;

    fn new(item: T) -> Self {
        BaggedItem {item}
    }
    fn get(&self) -> &T {
        &self.item
    }
}

fn main() {
    let boxed = BoxedItem::new(5);
    let bagged = BaggedItem::new("Hello");

    println!("Boxed: {}", boxed.get());
    println!("Bagged: {}", bagged.get());
}
