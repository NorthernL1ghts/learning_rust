// Lesson 12: Lifetimes and References

fn main() {
    // Using references
    // Create a string and calculate its length using a reference
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Pass an immutable reference to s1

    // Print the original string and its length
    println!("The length of '{}' is {}.", s1, len);

    // Using mutable references
    // Create a mutable string
    let mut s = String::from("hello");

    // Modify the string using a mutable reference
    change(&mut s);

    // Print the modified string
    println!("The changed string is '{}'.", s);

    // Using lifetimes
    // Create two strings to compare
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");

    // Determine the longest string
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is '{}'", result);
}

// Function to calculate the length of a string using a reference
fn calculate_length(s: &String) -> usize {
    s.len() // Return the length of the string
}

// Function to change a string using a mutable reference
fn change(s: &mut String) {
    s.push_str(", world"); // Append to the original string
}

// Function to determine the longest of two string slices
// The lifetime annotation 'a ensures the returned reference is valid
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x // Return x if it is longer
    } else {
        y // Otherwise, return y
    }
}
