// Your function here
struct Point<T> {
    x: T,
    y: T,
}

// Do not modify main
fn main() {
    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    // Complete the following statements
    println!("Integer Point: ({}, {})", int_point.x, int_point.y);
    println!("Float Point: ({}, {})", float_point.x, float_point.y);
}
