use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::time::Instant;
use crate::cli::LineResult::{Output};
use crate::lang::run_line;

pub fn cli(memory: &mut HashMap<String, f64>) {
    println!("Malors CLI launched");
    println!("Commands: $m to print heap | $q to quit");
    loop {
        print!(">>> ");
        let mut input = String::new();
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut input).expect("INTERNAL-ERROR: Failed to get user input");
        if input.trim() == "$q" {
            break;
        } else if input.trim().is_empty() {
            // Do nothing
        } else if input.trim() == "$m" {
            println!("Memory state:\n {:?}", memory.clone())
        } else {
            let start_time = Instant::now();
            let result = run_line(input.as_str(), memory);
            if let Ok(Output(string)) = result {
                println!("{}", string)
            } else if let Err(err) = result{
                println!("\x1b[31mPROGRAM ERROR:\x1b[0m \n{}", err);
                println!("<Instruction skipped>");
            }
            let end_time = Instant::now();
            let elapsed_time = end_time - start_time;
            memory.insert("_ms".into(), elapsed_time.as_millis() as f64);
        }
    }
    println!("The End.")
}

pub enum LineResult {
    Output(String),
    Nothing
}