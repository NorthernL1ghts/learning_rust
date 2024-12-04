// Lesson 14: Asynchronous Programming

use std::time::Duration;
use tokio::time::sleep;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
use tokio::task;

// Define an asynchronous function
async fn say_hello() {
    // Sleep for 2 seconds asynchronously
    sleep(Duration::from_secs(2)).await;
    println!("Hello, world!");
}

// Define an asynchronous function to read a file
async fn read_file(path: &str) -> io::Result<String> {
    // Open the file asynchronously
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    // Read the contents of the file asynchronously
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

// Define an asynchronous function for task one
async fn task_one() {
    // Sleep for 1 second asynchronously
    sleep(Duration::from_secs(1)).await;
    println!("Task one completed!");
}

// Define an asynchronous function for task two
async fn task_two() {
    // Sleep for 2 seconds asynchronously
    sleep(Duration::from_secs(2)).await;
    println!("Task two completed!");
}

#[tokio::main]
async fn main() {
    // Call the async function and await its completion
    say_hello().await;

    // Call the read_file function and handle the result
    match read_file("hello.txt").await {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }

    // Spawn two asynchronous tasks
    let handle1 = task::spawn(task_one());
    let handle2 = task::spawn(task_two());

    // Await the completion of both tasks
    handle1.await.unwrap();
    handle2.await.unwrap();
}
