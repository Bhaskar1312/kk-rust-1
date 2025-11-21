// Required imports here
use std::sync::Mutex;
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0)); // Mutex protected variable initialized with 0
    let mut handles = vec![]; // Empty mutable vector

    for _ in 0..10 { // Run the loop 10 times
        let counter = Arc::clone(&counter); // Clone the counter variable using Arc
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // Lock the mutex
            // Increment counter by 1
            *num += 1;
        });
        handles.push(handle); // Push the thread
    }

    for handle in handles {
        // Wait for all the spawned threads handle in vector handles to finish
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); // Print the result by locking the mutex again to retrieve the final value of counter
}
