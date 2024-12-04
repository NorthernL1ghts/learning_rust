// Lesson 4: Ownership, Borrowing, and References

fn main() {
    // Ownership
    let s1 = String::from("hello");
    let s2 = s1; // s1's value is moved to s2
    // s1 is no longer valid here; it's been moved
    // println!("{}", s1); // This line would cause a compile-time error
    println!("{}", s2);

    // Borrowing
    let s3 = String::from("hello");

    // Immutable borrow
    let len = calculate_length(&s3);
    println!("The length of '{}' is {}", s3, len);

    // Mutable borrow
    let mut s4 = String::from("hello");
    change(&mut s4);
    println!("{}", s4);

    // References and Slices
    let s5 = String::from("hello world");
    let hello = &s5[0..5];
    let world = &s5[6..11];
    println!("{} {}", hello, world);
}

// Function to calculate the length of a string
fn calculate_length(s: &String) -> usize {
    s.len()
}

// Function to change a mutable string by adding a suffix
fn change(s: &mut String) {
    s.push_str(", world");
}
