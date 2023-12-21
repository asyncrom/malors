mod tokenizer;
mod runner;
mod calculator;
mod line_type;

use std::collections::HashMap;
use crate::cli::LineResult;
use crate::lang::line_type::construct_line_type;
use crate::lang::runner::run;
use crate::lang::tokenizer::tokenize2;


pub fn run_line(line: &str, memory: &mut HashMap<String, f64>) -> Result<LineResult, String> {
    // Transform the line string into tokens
    let tokens = tokenize2(line)?;
    //DEBUG print tokens
    //println!("{:?}", tokens);
    let line_type = construct_line_type(tokens)?;
    //println!("{:?}", line_type);
    let line_result = run(memory, line_type)?;
    return Ok(line_result)

    // println!("{}", calculate(tokens));
    // LineResult::none()
}