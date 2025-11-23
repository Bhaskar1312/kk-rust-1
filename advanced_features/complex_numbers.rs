use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
struct ComplexNumber {
    real: f64,
    imag: f64,
}

impl Add for ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, other: ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

fn main() {
    let num1 = ComplexNumber { real: 1.0, imag: 2.0 };
    let num2 = ComplexNumber { real: 3.0, imag: 4.0 };
    let result = num1 + num2;
    println!("Result: {:?}", result);
}