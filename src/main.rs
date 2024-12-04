// Lesson 10: Concurrency

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

fn main() {
    // Using threads
    // Create a new thread that will print messages
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1)); // Simulate work by sleeping
        }
    });

    // Run code in the main thread
    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1)); // Simulate work by sleeping
    }

    // Wait for the spawned thread to finish execution
    handle.join().unwrap();

    // Using message passing
    // Create a channel for sending messages between threads
    let (tx, rx) = mpsc::channel();
    
    // Spawn a new thread and move the transmitter (tx) into it
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // Send a message through the channel
    });

    // Receive the message in the main thread
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // Using shared state with Arc and Mutex
    // Create an Arc (Atomic Reference Counted) and Mutex to safely share and modify data across threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // Spawn multiple threads to increment the counter
    for _ in 0..10 {
        let counter = Arc::clone(&counter); // Clone the Arc to increase the reference count
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // Acquire the lock on the Mutex
            *num += 1; // Increment the counter
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the result
    println!("Result: {}", *counter.lock().unwrap());
}
