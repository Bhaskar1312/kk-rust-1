use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");
            let msg = b"Hello from client!";
            stream.write(msg).unwrap();
            println!("Sent message from client:");
            let mut buffer: [u8; 512]= [0; 512];
            // loop {
                match stream.read(&mut buffer) {
                    Ok(n) => {
                        println!("Received: {}", String::from_utf8_lossy(&buffer[0..n])); // Print received data
                    }
                    Err(e) => {
                        println!("Error reading from stream: {}", e);
                       
                    }
                }
            // }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
