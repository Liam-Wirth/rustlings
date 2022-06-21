// move_semantics6.rs
// Make me compile! `rustlings hint move_semantics6` for hints
// You can't change anything except adding or removing references


//TODO what is the difference between String &String str &str
//! The lesson learned here is that when a variable is called and prefixed with &, then you are borrowing the variable temporarily
//! and when passing a variable to a function without that prefix
//! the function will take ownership of that variable
fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data.to_string());

    string_uppercase(data);
}
// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
