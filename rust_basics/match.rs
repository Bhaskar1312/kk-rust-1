// match similar to switch in other languages
fn main() {
    let number = 3;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 | 5 => println!("Four or Five"),
        6..=10 => println!("Between Six and Ten"),
        _ => println!("Other number"),
    }

}