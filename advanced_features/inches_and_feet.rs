use std::ops::Add;

struct Inches(u32);
struct Feet(u32);

// Add feet + inches = inches
impl Add<Feet> for Inches {
    type Output = Inches;

    fn add(self, other: Feet) -> Inches {
        Inches(self.0 + other.0 * 12)
    }
}

fn main() {
    let length_in_inches = Inches(10);
    let length_in_feet = Feet(2);
    
    let result = length_in_inches + length_in_feet; // inches(self) + feet(other)
    println!("Total length in inches: {}", result.0);
}
