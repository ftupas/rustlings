// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let a = 0..100; // creating an array from 0 to 99
    let b = ["Are we there yet"; 10]; // Another shorthand notation for creating an array

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
        println!("{:?}", b); // This is how to print an array
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
