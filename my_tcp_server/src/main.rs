use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break, // Connection was closed by the client
            Ok(n) => {
                stream.write(&buffer[0..n]).unwrap(); // Echo back the received data
                println!("Received: {}", String::from_utf8_lossy(&buffer[0..n])); // Print received data
            }
            Err(e) => {
                println!("Error reading from stream: {}", e);
                break;
            }

        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on port 7878");

    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream);
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}
