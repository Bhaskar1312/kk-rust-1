use tokio::time::{sleep, Duration};

async fn simple_delay() {
    sleep(Duration::from_secs(2)).await;
    println!("Delay complete!");
}
#[tokio::main]
async fn main() {
    simple_delay().await;
}
