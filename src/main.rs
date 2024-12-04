// Lesson 6: Enums and Pattern Matching

// Define an enum for different types of coins
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

// Define another enum for US states
#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
    // add more states as needed
}

// Define an enum for different message types
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Function to process messages based on their type
fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit message received."),
        Message::Move { x, y } => println!("Move message to ({}, {}).", x, y),
        Message::Write(text) => println!("Write message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {}).", r, g, b),
    }
}

// main function
fn main() {
    // Using all Coin variants
    let coin1 = Coin::Penny;
    let coin2 = Coin::Nickel;
    let coin3 = Coin::Dime;
    let coin4 = Coin::Quarter(State::Alabama);
    let coin5 = Coin::Quarter(State::Alaska); // Adding Alaska variant

    match coin1 {
        Coin::Penny => println!("This is a penny."),
        Coin::Nickel => println!("This is a nickel."),
        Coin::Dime => println!("This is a dime."),
        Coin::Quarter(state) => println!("This is a quarter from {:?}.", state),
    }

    match coin2 {
        Coin::Penny => println!("This is a penny."),
        Coin::Nickel => println!("This is a nickel."),
        Coin::Dime => println!("This is a dime."),
        Coin::Quarter(state) => println!("This is a quarter from {:?}.", state),
    }

    match coin3 {
        Coin::Penny => println!("This is a penny."),
        Coin::Nickel => println!("This is a nickel."),
        Coin::Dime => println!("This is a dime."),
        Coin::Quarter(state) => println!("This is a quarter from {:?}.", state),
    }

    match coin4 {
        Coin::Penny => println!("This is a penny."),
        Coin::Nickel => println!("This is a nickel."),
        Coin::Dime => println!("This is a dime."),
        Coin::Quarter(state) => println!("This is a quarter from {:?}.", state),
    }

    match coin5 {
        Coin::Penny => println!("This is a penny."),
        Coin::Nickel => println!("This is a nickel."),
        Coin::Dime => println!("This is a dime."),
        Coin::Quarter(state) => println!("This is a quarter from {:?}.", state),
    }

    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello"));
    let msg4 = Message::ChangeColor(255, 0, 0);

    process_message(msg1);
    process_message(msg2);
    process_message(msg3);
    process_message(msg4);
}
