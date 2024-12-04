// Lesson 1: Variables and Basic Data Types

fn main() {
    // Immutable variable
    let x = 5;
    println!("The value of x is: {}", x);

    // Mutable variable
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 20;
    println!("The new value of y is: {}", y);

    // Basic Data Types

    // Integer
    let int: i32 = 42;
    println!("Integer: {}", int);

    // Floating-point number
    let float: f64 = 3.14;
    println!("Float: {}", float);

    // Boolean
    let is_true: bool = true;
    println!("Boolean: {}", is_true);

    // Character
    let letter: char = 'R';
    println!("Character: {}", letter);

    // Tuple
    let tuple: (i32, f64, char) = (500, 6.4, 'T');
    println!("Tuple: ({}, {}, {})", tuple.0, tuple.1, tuple.2);

    // Array
    let array: [i32; 4] = [1, 2, 3, 4];
    println!("Array: {:?}", array);
}
