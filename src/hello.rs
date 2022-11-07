// This module prints a greeting in the console
// Author: Adán G. Medrano-Chávez

use std::io; // Accesses the standard output

// Rust function looks like Python's function
pub fn hello(greeting: &str) {
    println!("Who are you?");
    let mut name: String = String::new();
    // passing name to read_line as reference
    io::stdin().read_line(&mut name)
        .expect("User does not input any name");
    // println works like Python's print fn
    println!("{}, {}", greeting, name);
}