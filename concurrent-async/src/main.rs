
async fn concurrent_task() {
    println!("Running concurrently");
}

#[tokio::main]
async fn main() {
   let handle = tokio::spawn(concurrent_task());
   handle.await.unwrap();
   println!("Task spawned");
}
