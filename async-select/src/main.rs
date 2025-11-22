use tokio::time::{sleep, Duration};
use tokio::select;

async fn fast_task() {
    sleep(Duration::from_secs(1)).await;
    println!("Fast task completed");
}

async fn slow_task() {
    sleep(Duration::from_secs(3)).await;
    println!("Slow task completed");
}

#[tokio::main]
async fn main() {
    select! {
        _ = fast_task() => println!("Fast task finished first"),
        _ = slow_task() => println!("Slow task finished first"),
    }
}
