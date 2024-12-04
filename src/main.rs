// Lesson 8: Hash Maps

use std::collections::HashMap;

fn main() {
    // Creating an empty hash map
    let mut scores = HashMap::new();

    // Adding key-value pairs to the hash map
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Scores: {:?}", scores);

    // Creating a hash map with initial key-value pairs
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("Initial scores: {:?}", scores);

    // Accessing values in a hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    match scores.get(&team_name) {
        Some(score) => println!("The score for {} is {}", team_name, score),
        None => println!("No score found for {}", team_name),
    }

    // Iterating over a hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating values in a hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // Overwriting a value
    scores.insert(String::from("Blue"), 25);
    println!("Updated score for Blue: {:?}", scores.get("Blue"));

    // Using the entry method to insert a value if the key does not exist
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("Scores after inserting Yellow: {:?}", scores);

    // Using the entry method to update a value if the key exists
    scores.entry(String::from("Blue")).and_modify(|e| *e += 10);
    println!("Scores after modifying Blue: {:?}", scores);
}
