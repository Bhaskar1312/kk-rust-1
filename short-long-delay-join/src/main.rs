use tokio::time::{sleep, Duration};
use tokio::join;

async fn short_delay() {
    sleep(Duration::from_secs(1)).await;
    println!("Short delay complete");
}

async fn long_delay() {
    sleep(Duration::from_secs(3)).await;
    println!("Long delay complete");
}

#[tokio::main]
async fn main() {
    join!(short_delay(), long_delay());
    println!("All tasks completed");
}
