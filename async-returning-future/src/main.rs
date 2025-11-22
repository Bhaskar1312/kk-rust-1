
async fn return_value() -> &'static str {
    "Hello, Async!"
}

#[tokio::main]
async fn main() {
    let value = return_value().await;
    println!("{}", value);
}
