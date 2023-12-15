mod tokenizer;
mod runner;

use std::collections::HashMap;
use crate::cli::LineResult;
use crate::lang::runner::compile_line;
use crate::lang::tokenizer::tokenize;

pub fn run_line(line: &str, memory: &mut HashMap<String, f64>) -> LineResult {

    let mut tokens = tokenize(line);
    compile_line(memory, tokens.clone());
    LineResult::out(format!("line tokens:{:?}", tokens))
}
