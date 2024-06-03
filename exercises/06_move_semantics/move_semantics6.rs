// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); // Should be a reference to String

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char { // Should take in &String
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) { // Take mutable ownership to data
    data = data.to_uppercase(); // Remove reference

    println!("{}", data);
}
