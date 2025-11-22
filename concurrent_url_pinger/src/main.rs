// future - represent a value that may not be available yet
// Task - execution of a future
use reqwest;
use tokio::task;
use std::sync::Arc;
use tokio::sync::Semaphore;

async fn ping_url(url: &str) -> Result<reqwest::StatusCode, reqwest::Error> {
    let response = reqwest::get(url).await?;
    Ok(response.status())
}

async fn ping_multiple_urls(urls: Vec<&str>, max_concurrent: usize) {
    let semaphore = Arc::new(Semaphore::new(max_concurrent));
    let mut handles = vec![];

    for url in urls {
        let url = url.to_string();
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let handle = task::spawn(async move {
            match ping_url(&url).await {
                Ok(status) => println!("{} -> {}", url, status),
                Err(e) => println!("{} -> Error: {}", url, e),
            }
        });
        drop(permit);
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
        "https://www.github.com/",
        "https://www.stackoverflow.com/",
        "https://www.reddit.com/",
        "https://www.wikipedia.org/",
        "https://www.microsoft.com/",
        "https://www.apple.com/",
        "https://www.linkedin.com/",
        "https://www.twitter.com/",
        "https://www.facebook.com/",
        "https://www.instagram.com/",
        "https://www.netflix.com/",
        "https://www.amazon.com/",
        
    ];

    println!("Pinging URLs concurrently...");
    ping_multiple_urls(urls, 8).await;
    println!("Done.");
}
