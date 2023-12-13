use  std::env;
use crate::cli::cli;

mod cli;
mod lang;

fn main() {
    println!("Malors = Mathematic Logic from Rust.simplify()");
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if there are no arguments (other than the program name)
    if args.len() == 1 {
        cli()
    }
}