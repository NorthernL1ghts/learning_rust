// Lesson 16: Error Handling

use std::fs::File;
use std::io::{self, Read};

// Define a function to read the content of a file
// The function returns a Result with the file content as a String, or an io::Error
fn read_file_content(filename: &str) -> Result<String, io::Error> {
    // Attempt to open the file
    let mut file = File::open(filename)?;
    let mut content = String::new();
    // Attempt to read the file's content into the string
    file.read_to_string(&mut content)?;
    // Return the file content if successful
    Ok(content)
}

// Define a function to find a word in a string
// The function returns an Option with the index of the word, or None if not found
fn find_word(s: &str, word: &str) -> Option<usize> {
    s.find(word)
}

fn main() {
    // Handle the result of reading a file
    match read_file_content("hello.txt") {
        // If successful, print the file content
        Ok(content) => println!("File content:\n{}", content),
        // If there is an error, print the error message
        Err(e) => println!("Error reading file: {}", e),
    }

    // Define a sample sentence
    let sentence = "The quick brown fox jumps over the lazy dog.";
    // Handle the result of finding a word in the sentence
    match find_word(sentence, "fox") {
        // If the word is found, print the index
        Some(index) => println!("Found 'fox' at index {}", index),
        // If the word is not found, print a message
        None => println!("'fox' not found"),
    }
}
