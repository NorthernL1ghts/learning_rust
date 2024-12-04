// Lesson 11: Smart Pointers

fn main() {
    // Using Box<T>
    // Create a box to store data on the heap
    let b = Box::new(5);
    println!("b = {}", b);

    // Using Rc<T>
    // Rc<T> enables multiple ownership by keeping a reference count
    use std::rc::Rc;

    // Create a reference-counted integer
    let a = Rc::new(5);
    // Clone the Rc to create additional references
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    // Print the reference count and values
    println!("Reference count: {}", Rc::strong_count(&a));
    println!("a = {}, b = {}, c = {}", a, b, c);

    // Using RefCell<T>
    // RefCell<T> allows interior mutability with runtime borrow checking
    use std::cell::RefCell;

    // Create a RefCell containing an integer
    let x = RefCell::new(5);

    // Borrow a mutable reference to the contained value
    {
        let mut y = x.borrow_mut();
        *y += 1; // Modify the value inside the RefCell
    }

    // Print the modified value
    println!("x = {:?}", x);
}
