mod geometry;
mod math;

fn main() {
    let sum = crate::math::add(5, 10);
    let area = geometry::area(4);
    println!("Sum: {}", sum);
    println!("Area: {}", area);
}