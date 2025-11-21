// Required imports here
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel(); // Create a channel using mpsc

    for i in 0..5 {
        let tx_clone = tx.clone();  // Hold the clone of tx
        thread::spawn( move || { // Complete this line
            tx_clone.send(i).unwrap(); // Send loop index to main
        });
    }

    for received in rx.iter().take(5) { // Take only 5 messages
        println!("Received: {}", received);  // Complete this
    }
}
