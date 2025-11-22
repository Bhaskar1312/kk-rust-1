// future - represent a value that may not be available yet
// Task - execution of a future
use reqwest;
use tokio::task;

async fn ping_url(url: &str) -> Result<reqwest::StatusCode, reqwest::Error> {
    let response = reqwest::get(url).await?;
    Ok(response.status())
}

async fn ping_multiple_urls(urls: Vec<&str>) {
    let mut handles = vec![];

    for url in urls {
        let url = url.to_string();
        let handle = task::spawn(async move {
            match ping_url(&url).await {
                Ok(status) => println!("{} -> {}", url, status),
                Err(e) => println!("{} -> Error: {}", url, e),
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.await;
    }
}

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://www.rust-lang.org/",
        "https://www.example.com/",
        "https://www.google.com/",
        "https://nonexistent.url/",
    ];

    println!("Pinging URLs concurrently...");
    ping_multiple_urls(urls).await;
    println!("Done.");
}
