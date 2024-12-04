// Lesson 13: Traits and Generics

// Define a trait called Summary
trait Summary {
    fn summarize(&self) -> String;
}

// Implement the Summary trait for the NewsArticle struct
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({}) - {}", self.headline, self.author, self.location, self.content)
    }
}

fn main() {
    // Create a NewsArticle instance
    let article = NewsArticle {
        headline: String::from("Breaking News!"),
        location: String::from("Somewhere"),
        author: String::from("Someone"),
        content: String::from("Something important happened."),
    };

    // Print the summary of the article using the Summary trait method
    println!("New article available! {}", article.summarize());

    // Using generics in a function
    // Create a list of integers
    let number_list = vec![34, 50, 25, 100, 65];
    // Find the largest number in the list using the generic function
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    // Create a list of characters
    let char_list = vec!['y', 'm', 'a', 'q'];
    // Find the largest character in the list using the same generic function
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// Define a generic function to find the largest element in a list
// The function works with any type T that implements the PartialOrd trait
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0]; // Assume the first element is the largest initially

    for item in list.iter() { // Iterate over the list
        if item > largest { // Update the largest element if the current item is greater
            largest = item;
        }
    }

    largest // Return the largest element found
}
