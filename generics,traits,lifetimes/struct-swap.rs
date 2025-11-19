// Your enum here
struct Pair<R> {
    first: R,
    second: R,
}
// Your function implementation here
impl<R> Pair<R> {
    fn swap(self) -> Pair<R> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

// Do not modify
fn main() {
    let pair = Pair { first: 1, second: 2 };
    let swapped_pair = pair.swap();
    println!("Swapped Pair: ({}, {})", swapped_pair.first, swapped_pair.second);
}
