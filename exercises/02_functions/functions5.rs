// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num // Function should have a return value because it has a return type
              // If it returns nothing, it implicitly returns () (UNIT type)
              // return num * num; explicitly returns
}
