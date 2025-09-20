fn main() {
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result {}", result),
        Err(e) => println!("Error: {}", e),
    };

    match divide(10.0, 0.0) {
        Ok(result) => println!("Result {}", result),
        Err(e) => println!("Error: {}", e),
    };
}

fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("Cannot divide by zero")
    } else {
        Ok(a/b)
    }
}

