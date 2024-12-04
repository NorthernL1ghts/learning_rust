// Lesson 5: Structs and Methods

fn main() {
    // Creating a new user
    let mut user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
        active: true,
    };

    // Using the email field to avoid the warning
    println!("User's email: {}", user1.email);

    // Calling methods on the user
    user1.sign_in();
    user1.deactivate();
    println!("User active status: {}", user1.active);
    println!("{} has signed in {} times.", user1.username, user1.sign_in_count);
}

// Define a User struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Implement methods for the User struct
impl User {
    // Method to increment the sign-in count
    fn sign_in(&mut self) {
        self.sign_in_count += 1;
        println!("{} has signed in {} times.", self.username, self.sign_in_count);
    }

    // Method to deactivate the user
    fn deactivate(&mut self) {
        self.active = false;
    }
}
