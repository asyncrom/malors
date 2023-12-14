use std::io;
use std::io::Write;
use crate::cli::LineResultType::{Error, Out};
use crate::lang::run_line;

pub fn cli() {
    println!("Malors CLI launched");
    loop {
        print!(">>> ");
        let mut input = String::new();
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut input).expect("INTERNAL-ERROR: Failed to get user input");
        if input.trim() == "quit" {
            break;
        } else if input.trim().is_empty() {
            // Do nothing
        } else {
            let result = run_line(input.as_str());
            if result.res_type == Out {
                println!("{}", result.string)
            } else if result.res_type == Error {
                println!("PROGRAM ERROR:\n{}", result.string)
            }
        }
    }
    println!("The End.")
}

pub struct LineResult {
    string: String,
    res_type: LineResultType,
}

impl LineResult {
    pub fn none() -> LineResult {
        LineResult {
            string: "".to_string(),
            res_type: LineResultType::None,
        }
    }

    pub fn out(str: String) -> LineResult {
        LineResult {
            string: str.to_string(),
            res_type: LineResultType::Out,
        }
    }

    pub fn error(str: &str) -> LineResult {
        LineResult {
            string: str.to_string(),
            res_type: LineResultType::Error,
        }
    }
}

#[derive(PartialEq)]
pub enum LineResultType {
    None,
    Out,
    Error,
}