// Required imports here
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();  // Create mpsc channel
    let rx = Arc::new(Mutex::new(rx)); // Wrap the receiver in Arc and Mutex for sharing

    // Send tasks to the channel
    for i in 0..10 {
        let tx_clone = tx.clone();  // Clone the transmitter
        tx_clone.send(i).unwrap();  // Send loop index to main thread
    }

    let mut handles = vec![];  // Empty vector

    // Spawn 4 threads to process tasks from the same receiver
    for _ in 0..4 {
        let rx = Arc::clone(&rx); // Clone the Arc to share the receiver across threads
        let handle = thread::spawn(move || {    // Start thread
            while let Ok(task) = rx.lock().unwrap().recv() {  // Receive messages continuously
                println!("Processing task: {}", task);
            }
        });
        handles.push(handle);  // Push thread to vector
    }

    // Wait for all threads to finish
    for handle in handles{
        handle.join().unwrap();
    }
}
// Complete a RUST program workq-with-multi-consumers.rs that simulates a work queue with multiple consumers. Use a channel to send 10 tasks to 4 worker threads. Each worker should process tasks and print the task it is processing.