use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let url = "https://httpbin.org/get";
    let response = reqwest::get(url).await?;
    println!("Response: {:?}", response.text().await?);
    Ok(())
}
