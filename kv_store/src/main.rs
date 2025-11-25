use std::path::Component::ParentDir;
use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
mod command;
use command::Command;
mod store;
mod persistence;

use std::sync::Arc;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    println!("Server listening on port 6379");

    let store = Arc::new(store::KvStore::new());
    store.load("data.json").await?;

    let store_clone = store.clone();
    // graceful shutdown handler
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        store_clone.save("data.json").await.unwrap();
        std::process::exit(0);
    });

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New client: {:?}", addr);
        let store = store.clone();
        

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, store).await {
                eprintln!("Error handling client {}: {:?}", addr, e);
            }
        });
    }
}

async fn handle_client(socket: tokio::net::TcpStream, store: Arc<store::KvStore>) -> anyhow::Result<()> {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await?;
        if bytes_read == 0 {
            break; // Connection closed
        }
        println!("Received: {}", line.trim());
        // for now echo the message back
        // writer.write_all(line.as_bytes()).await?;

        let command = Command::parse(&line);
        match command {
            Command::Get(key) => {
                // handle GET command
                if let Some(value) = store.get(&key).await {
                    writer.write_all(format!("{}\r\n", value).as_bytes())
                        .await?;
                } else {
                    writer.write_all(b"(nil)\r\n").await?;
                }
                // writer.write_all(b"OK\n").await?;
            },
            Command::Set(key, value) => {
                // handle SET command
                // writer.write_all(b"OK\n").await?;
                store.set(key, value).await;
                writer.write_all(b"OK\n").await?;
            },
            Command::Delete(key) => {
                // handle DELETE command
                // writer.write_all(b"OK\n").await?;
                match store.delete(&key).await {
                    Ok(true) => writer.write_all(b"OK\n").await?,
                    Ok(false) => writer.write_all(b"(nil)\n").await?,
                    Err(e) => {
                        writer.write_all(format!("{}\n", e).as_bytes()).await?;
                    }
                }

            },
            Command::Unknown => {
                writer.write_all(b"ERR unknown command\n").await?;
            }
        }
    }
    Ok(())
}
// cargo run
// telnet localhost 6379
// SET x 100
// OK
// GET x
// 100
// DELETE x
// OK
// GET x
// (nil)