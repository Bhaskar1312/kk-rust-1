// Your enum here
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    // Your vars here
    let success: MyResult<i32, &str> = MyResult::Ok(42);
    let error: MyResult<i32, &str> = MyResult::Err("An error occurred");
    // Do not modify
    match success {
        MyResult::Ok(val) => println!("Success with value: {}", val),
        MyResult::Err(err) => println!("Error: {}", err),
    }

    match error {
        MyResult::Ok(val) => println!("Success with value: {}", val),
        MyResult::Err(err) => println!("Error: {}", err),
    }
}
