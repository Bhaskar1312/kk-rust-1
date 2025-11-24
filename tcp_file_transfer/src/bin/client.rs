use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::io::stdin;

fn get_file_path_from_user() -> String {
    println!("Please enter the absolute path of the file to send to:");
    let mut file_path = String::new();
    stdin().read_line(&mut file_path).expect("Failed to read line");
    file_path.trim().to_string()
}

fn send_file(file_path: &str) -> std::io::Result<()> {
    let mut file = File::open(file_path)?;
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("Connected to server at 127.0.0.1:8080");
    
    let mut buffer = [0u8; 1024];
    loop {
        let n = match file.read(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read from file: {}", e);
                return Err(e);
            }
        };
        stream.write_all(&buffer[..n])?;
        println!("Sent {} bytes", n);
    }
    // Ensure that connection is properly closed after sending the file
    stream.shutdown(std::net::Shutdown::Write)?;

    let mut response = [0u8; 1024];
    match stream.read(&mut response) {
        Ok(n) => {
            // print ack received from server
            println!("Server response: {}", String::from_utf8_lossy(&response[..n]));
        }
        Err(e) => {
            eprintln!("Failed to read acknowledgment from server: {}", e);
        }
    }
    Ok(())
}

fn main() {
    let file_path = get_file_path_from_user();
    match send_file(&file_path) {
        Ok(_) => println!("File sent successfully."),
        Err(e) => eprintln!("Error sending file: {}", e),
    }
}
//  cargo run --bin client
// Example absolute path on macOS/Linux: /Users/username/Documents/file.txt or file.txt