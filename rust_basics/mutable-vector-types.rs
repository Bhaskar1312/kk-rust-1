// Your enum here
enum Data {
    Integer(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // Your vector here
    let mut data = vec![
        Data::Integer(42),
        Data::Float(3.14),
        Data::Text(String::from("Hello")),
    ];

    for item in data {
        match item {
            Data::Integer(i) => println!("Integer: {}", i),
            Data::Float(f) => println!("Float: {}", f),
            Data::Text(s) => println!("Text: {}", s),
        }
    }
}
