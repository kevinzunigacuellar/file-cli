use std::env;
use std::fs;

fn main() {
    println!("Welcome to my cli app!");
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file = &args[2];

    println!("Searching for '{}' in file: {}", query, file);

    let content = fs::read_to_string(file).expect("Something went wrong reading the file");

    println!("With text:\n{}", content);
}
