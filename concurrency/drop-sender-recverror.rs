use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();
    let handle = thread::spawn(move || {
        drop(tx);
    });
    handle.join().unwrap();
    match rx.recv() {
        Ok(msg) => println!("Received: {}",msg),
        Err(e) => println!("All senders dropped, stopping."),
    }
}