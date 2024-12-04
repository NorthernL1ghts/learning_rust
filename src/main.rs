// Lesson 18: File I/O Operations

use std::fs::File;
use std::io::{self, Read, Write};
use std::fs::OpenOptions;

// Function to read the content of a file
fn read_file(filename: &str) -> io::Result<String> {
    // Open the file for reading
    let mut file = File::open(filename)?;
    let mut content = String::new();
    // Read the file contents into a string
    file.read_to_string(&mut content)?;
    // Return the content
    Ok(content)
}

// Function to write content to a file
fn write_file(filename: &str, content: &str) -> io::Result<()> {
    // Create or truncate the file for writing
    let mut file = File::create(filename)?;
    // Write the content to the file
    file.write_all(content.as_bytes())?;
    // Return Ok if successful
    Ok(())
}

// Function to append content to a file
fn append_to_file(filename: &str, content: &str) -> io::Result<()> {
    // Open the file with append mode
    let mut file = OpenOptions::new()
        .append(true)
        .open(filename)?;
    // Write the content to the file
    file.write_all(content.as_bytes())?;
    // Return Ok if successful
    Ok(())
}

fn main() {
    // Specify the file name
    let filename = "example.txt";

    // Write content to the file
    let content = "Hello, Rustaceans!";
    match write_file(filename, content) {
        Ok(_) => println!("Successfully wrote to the file."),
        Err(e) => println!("Error writing to file: {}", e),
    }

    // Read the file and handle the result
    match read_file(filename) {
        Ok(content) => println!("File content:\n{}", content),
        Err(e) => println!("Error reading file: {}", e),
    }

    // Append content to the file
    let append_content = "\nLet's append this text.";
    match append_to_file(filename, append_content) {
        Ok(_) => println!("Successfully appended to the file."),
        Err(e) => println!("Error appending to file: {}", e),
    }

    // Read the file again to see the appended content
    match read_file(filename) {
        Ok(content) => println!("Updated file content:\n{}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
}
