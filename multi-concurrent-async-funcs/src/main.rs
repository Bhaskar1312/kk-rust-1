use tokio::time::{sleep, Duration};
use tokio::join;
async fn task_one() {
    sleep(Duration::from_secs(2)).await;
    println!("Task one completed");
}

async fn task_two() {
    sleep(Duration::from_secs(3)).await;
    println!("Task two completed");
}

#[tokio::main]
async fn main() {
    join!(task_one(), task_two());
    println!("Both tasks completed");
}
