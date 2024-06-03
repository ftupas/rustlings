// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    // Add a new block and use shadowing to redeclare a variable that lives on this specific scope
    {
        let number = 3; // don't rename this variable
        println!("Number plus two is : {}", number + 2);
    }
    // As scope ends, number is back to the original value
    println!("Number is : {}", number);
}
