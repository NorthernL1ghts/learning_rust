// Lesson 3: Functions

fn main() {
    // Calling the greet function
    greet("Rustacean");

    // Calling the add function and printing the result
    let result = add(5, 3);
    println!("The sum is: {}", result);

    // Calling the square function and printing the result
    let square_result = square(4);
    println!("The square of 4 is: {}", square_result);

    // Calling the calculate_area function and printing the result
    let width = 10;
    let height = 5;
    let area = calculate_area(width, height);
    println!("The area of the rectangle is: {}", area);
}

// Defining a function to greet the user
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Function with parameters and a return value to add two numbers
fn add(a: i32, b: i32) -> i32 {
    a + b // Return statement
}

// Function that returns the square of a number
fn square(x: i32) -> i32 {
    x * x // This is an expression, so no semicolon
}

// Function to calculate the area of a rectangle
fn calculate_area(width: i32, height: i32) -> i32 {
    width * height
}
