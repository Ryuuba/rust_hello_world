use std::io;
use std::env;

fn hello(greeting: &str) {
    println!("Who are you?");
    let mut name: String = String::new();
    // passing name to read_line as reference
    io::stdin().read_line(&mut name)
        .expect("User does not input any name");
    // println works like Python's print
    println!("{}, {}", greeting, name);
}

fn main() {
    // In this example, collect convert Args-type iterator into a vector of
    // strings, otherwise, manipulating command-line arguments becomes awkward
    let argv: Vec<String> = env::args().collect();
    // as_str returns a &str object from a String
    match argv[1].as_str() { //match needs a &str object
        "hello" => hello(argv[2].as_str()),
        _ => println!("invalid option")
    }
}
