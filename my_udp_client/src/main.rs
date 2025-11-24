use std::net::UdpSocket;
use std::io::Result;

fn main() -> Result<()>{
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    println!("UDP client started and bound to {}", socket.local_addr()?);

    let server_addr = "127.0.0.1:7878";
    let message = b"Hello, from UDP client!";
    socket.send_to(message, server_addr)?;
    println!("Sent message to server {}: {:?}", server_addr, String::from_utf8_lossy(message));

    let mut buf: [u8; 512] = [0; 512];
    let (bytes_received, src_address) = socket.recv_from(&mut buf)?;
    println!(
        "Received {} bytes from {}: {:?}",
        bytes_received,
        src_address,
        String::from_utf8_lossy(&buf[..bytes_received])
    );
    Ok(())
}
