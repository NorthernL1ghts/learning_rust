// Lesson 15: Foreign Function Interface (FFI)

extern crate libc;

use libc::c_int;

// Declare an external C function
extern "C" {
    fn abs(input: c_int) -> c_int;
}

fn main() {
    let number: c_int = -42;

    unsafe {
        // Call the C function
        let result = abs(number);
        println!("The absolute value of {} is {}", number, result);
    }
}
