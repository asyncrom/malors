mod tokenizer;
mod runner;

use crate::cli::LineResult;
use crate::lang::tokenizer::tokenize;

pub fn run_line(line: &str) -> LineResult {

    let mut tokens = tokenize(line);
    LineResult::out(format!("line tokens:{:?}", tokens))
}
