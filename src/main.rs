use std::collections::HashMap;
use  std::env;
use std::fs::File;
use std::io::Read;
use crate::cli::cli;

mod cli;
mod lang;

fn main() {
    // Initiate heap memory
    let mut memory:HashMap<String, f64> = HashMap::new();

    println!("Malors = Mathematic Logic from Rust.simplify()");
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if there are no arguments (other than the program name)
    cli(&mut memory)
}