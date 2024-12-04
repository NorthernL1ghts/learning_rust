// Lesson 2: Control Flow

fn main() {
    // Using if-else statements to make decisions based on conditions
    let number = 7;

    if number < 5 {
        println!("The number is less than 5");
    } else if number > 5 {
        println!("The number is greater than 5");
    } else {
        println!("The number is exactly 5");
    }

    // Using match statements for pattern matching
    let number = 3;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }

    // Using different types of loops

    // Infinite loop with break condition
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            break;
        }
        println!("Loop count: {}", count);
    }

    // While loop
    let mut num = 3;
    while num != 0 {
        println!("While num: {}", num);
        num -= 1;
    }

    // For loop iterating over an array
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("Array element: {}", element);
    }
}
