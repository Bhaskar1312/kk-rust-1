// Required imports here
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));  // Wrapped inside Arc and Mutex and initialized to 0
    let mut handles = vec![]; // Empty vector

    for i in 0..10 {   // Loop 10 times
        let counter = Arc::clone(&counter);    // Use Arc clone on counter
        let handle = thread::spawn(move || {  // Spawn a thread
            if i ==  5{   // Panic at 5
                panic!("Thread panicked!");
            }
            let mut num = counter.lock().unwrap();   // Acquire lock on shared counter
            *num += 1;   // Increment by 1
        });
        handles.push(handle);   // Push the thread
    }

    for handle in handles {  // Wait for all threads
        let _ = handle.join();  // Handle thread panic with join
    }

    match counter.lock() {   // Acquire lock on counter
        Ok(num) => println!("Result: {}", *num),   // Add appropriate variable
        Err(poisoned) => println!("The mutex is poisoned: {}", poisoned),  // Add appropriate variable
    };
}
// Complete a RUST program poisoned-mutex-multi-thread.rs that simulates a scenario where a thread panics while holding a Mutex. Ensure that the program can handle the poisoned mutex and continue processing in other threads.