mod tokenizer;
mod runner;
mod calculator;
mod line_type;

use std::collections::HashMap;
use crate::cli::LineResult;
use crate::lang::line_type::construct_line_type;
use crate::lang::tokenizer::{Compare, Operation, Operator, Token, tokenize};


pub fn run_line(line: &str, memory: &mut HashMap<String, f64>) -> LineResult {
    // Transform the line string into tokens
    let mut tokens = tokenize(line);
    //DEBUG print tokens
    println!("{:?}", tokens);
    let line_type = construct_line_type(tokens);
    println!("{:?}", line_type);

    return LineResult::none()
}