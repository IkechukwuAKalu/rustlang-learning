// each variable has an owner - and Rust removes them from memory when they go out of scope (the closing curly brace).
// scalar variables are able to clone/copy data by default because they implement the `copy` trait.
// other types such as String need to explicitly call the `clone` method on the variable to copy.
// string literals ("hello") is different from String types (String::from("hello")).
// data types with fixed size are stored on the stack while the rest are on the heap, with their pointer stored on the stack.
// passing data to functions as arguments also follow the same rules as assigning to another variable within the same function.
// only ONE mutable reference to a variable can exist at a time, but multiple immutable references can exist.
// you can have a mutable reference and multiple immutable references ONLY when the last usage of all immutable references come before the mutable reference.
// references must always be valid - no dangling references (ex. returning a refernce from a function that creates the value).
// Slice, a kind of reference for a contiguous sequence of elements in a collection, instead of the whole collection like String.
// Slices store a reference to the first element, and the length to be sliced.

fn main() {
    // move - default for non-scalar types
    let mut s1 = String::from("Hello");
    s1.push_str(", world!");

    let s2 = s1;

    // println!("{}", s1); - thie fails because s1 is invalidated
    println!("String 2 - {}", s2);

    // clone - expensive operation
    let mut s3 = String::from("Hello");
    s3.push_str(", world again!");

    let s4 = s3.clone();

    println!("String 3 - {}", s3);
    println!("String 4 - {}", s4);

    // copy - scalar values copy by default
    let x: u8 = 32;
    let y: u8 = x;

    println!("Scalar 1 - {}", x);
    println!("Scalar 2 - {}", y);

    // references - borrowing
    let msg_1 = String::from("References (Borrowing) test started");
    log_message_1(&msg_1);

    let mut msg_2 = String::from("References (Borrowing) test ended");
    log_message_2(&mut msg_2);

    // Slice
    let s5 = "Hello";
    println!("String 5 - {}", &s5[3..]);

    let s6 = String::from("World");
    println!("String 6 - {}", &s6[..]);

    let arr1 = [0, 2, 1, 3, 4, 5, 6];
    let _arr1_slice = &arr1[0..2];
}

// immutable reference sample
fn log_message_1(message: &String) {
    println!("{}", message);
}

// mutable reference sample
fn log_message_2(message: &mut String) {
    message.push_str(" *");
    println!("{}", message);
}
