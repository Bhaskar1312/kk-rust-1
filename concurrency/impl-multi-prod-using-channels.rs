// Required imports here
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();  // mpsc channel

    let messages = vec!["Message 1", "Message 2", "Message 3"]; // Vector holding messages

    for i in 0..3 {
        let tx_clone = tx.clone();  // Clone tx
        let msg = String::from(messages[i]);  // Hold ith string from messages
        thread::spawn(move || {
            tx_clone.send(msg).unwrap();  // Send local variable to main thread
        });
    }

    for received in rx.iter().take(3) {  // Process 3 messages
        println!("Received: {}", received);   // Complete the print statement
    }
}
