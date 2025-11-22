async fn error_handling_task() -> Result<(), &'static str> {
    Err("An error occurred")
}

#[tokio::main]
async fn main() {
    match error_handling_task().await {
        Ok(_) => println!("Task completed successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
     
}
