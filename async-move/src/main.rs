

#[tokio::main]
async fn main() {
    let data = vec![1, 2, 3];
    tokio::spawn(async move {
        println!("Data: {:?}", data);
    }).await.unwrap();
}
