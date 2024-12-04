// Lesson 9: Error Handling

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn main() {
    // Handling recoverable errors with Result
    match read_username_from_file("hello.txt") {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading file: {}", e),
    }

    // Handling unrecoverable errors with panic!
    let _v = vec![1, 2, 3];

    // This will cause the program to panic
    // Uncommenting the line below will terminate the program
    // _v[99];
}

fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
    let path = Path::new(filename);
    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }
    let mut file = File::open(&path)?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}
