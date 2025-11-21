use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let handle = 
    thread::spawn(move || {
        tx.send(String::from("Hello, world!")).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Received: {}", received);
    handle.join().unwrap();
}
