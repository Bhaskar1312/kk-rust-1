// Import your module here
mod math;

fn main() {
    let square = math::operations::advanced::square(4); // Call the square function from the module for value 4
    let cube = math::operations::advanced::cube(3); // Call the cube function from the module for value 3
    println!("Square of 4 is {}, Cube of 3 is {}", square, cube);
}
