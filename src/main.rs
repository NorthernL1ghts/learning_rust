// Lesson 7: Vectors

fn main() {
    // Creating an empty vector
    let mut v: Vec<i32> = Vec::new();

    // Adding elements to the vector
    v.push(1);
    v.push(2);
    v.push(3);

    // Creating a vector with initial elements
    let v2 = vec![1, 2, 3, 4, 5];

    println!("Vector v: {:?}", v);
    println!("Vector v2: {:?}", v2);

    // Accessing elements in a vector
    let v = vec![10, 20, 30, 40, 50];

    // Using indexing (may panic if the index is out of bounds)
    let third = &v[2];
    println!("The third element is {}", third);

    // Using the get method (returns an Option<&T>)
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // Iterating over a vector
    let v = vec![100, 200, 300, 400, 500];

    // Iterating with a for loop
    for i in &v {
        println!("Element: {}", i);
    }

    // Iterating with an iterator
    for i in v.iter() {
        println!("Element (iterator): {}", i);
    }

    // Modifying elements in a mutable vector
    let mut v = vec![1, 2, 3, 4, 5];

    // Iterating and modifying elements
    for i in &mut v {
        *i += 50;
    }

    println!("Modified vector: {:?}", v);
}
