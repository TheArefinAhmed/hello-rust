fn main() {
print_message("🐸".to_string()); // Correct: "🐸" (&str) is converted to String
}

fn print_message(s: String) { // Implicitly returns ()
    // In Rust 2021 edition and later, "{s}" uses s as an implicit named argument.
    // This line prints the content of the string 's' five times, concatenated.
    println!("{s}{s}{s}{s}{s}");
    println!("Hello World");
}