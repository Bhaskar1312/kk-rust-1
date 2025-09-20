fn main() {
    let mut arr = [1,2,3,4,5];
    let slice = &mut arr[1..4];

    slice[0] = 10;
    println!("{:?}", arr);
}