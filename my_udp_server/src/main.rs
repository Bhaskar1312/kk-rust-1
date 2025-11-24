use std::net::UdpSocket;
use std::io::Error;
use std::io::Result;

fn main() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:7878");
    let socket = match socket {
        Ok(s) => s, 
        Err(e) => {
            eprintln!("Failed to bind UDP socket: {}", e);
            return Err(e);
        },
    };
    println!("UDP server listening on 127.0.0.1:7878");

    let mut buf: [u8; 512] = [0; 512];
    loop {
        let (bytes_received, src_address) = socket.recv_from(&mut buf)?;
        println!(
            "Received {} bytes from {}: {:?}",
            bytes_received,
            src_address,
            &buf[..bytes_received]
        );
        // echo to client
        socket.send_to(&buf[..bytes_received], src_address)?; 
    }

}
