use std::collections::HashMap;
use std::ops::Add;
use crate::cli::LineResult;
use crate::lang::calculator::calculate;
use crate::lang::line_type::LineType;
use crate::lang::tokenizer::{Operation, Token};
use crate::lang::tokenizer::Token::Number;

pub fn run(memory: &mut HashMap<String, f64>, line_type: LineType) -> LineResult {
    match line_type {
        LineType::Nothing => {

        }
        LineType::Out(out) => {
            let mut result = "".to_string();
            for string in out {
                result.push_str(":");
                let num = memory.get(&string).expect("Var not found in memory");
                result.push_str(&*format!("{}", num));
            }
            return LineResult::out(result)
        }
        LineType::VarOperate(var, op, ex) => {
            let num = calculate(replace_var(memory,ex));
            match op {
                Operation::Assign => {
                    memory.insert(var, num);
                }
                Operation::AddVar => {
                    let ancient = memory.get(&var).expect("Var not found in memory");
                    memory.insert(var, ancient + num);
                }
                Operation::SubtractVar => {
                    let ancient = memory.get(&var).expect("Var not found in memory");
                    memory.insert(var, ancient - num);
                }
                Operation::MultiplyVar => {
                    let ancient = memory.get(&var).expect("Var not found in memory");
                    memory.insert(var, ancient * num);
                }
                Operation::DivideVar => {
                    let ancient = memory.get(&var).expect("Var not found in memory");
                    memory.insert(var, ancient / num);
                }
            }
        }
        LineType::If(_, _, _, _) => {}
        LineType::While(_, _, _, _) => {}
    }
    LineResult::none()
}

fn replace_var(memory: &mut HashMap<String, f64>, tokens : Vec<Token>) -> Vec<Token> {
    let mut tokens = tokens;
    for i in 0..tokens.len() {
        if let Token::Name(name) = tokens.get(i).unwrap() {
            tokens[i] = Number(*memory.get(name).expect("Var not found in memory"));
        }
    }
    return tokens;
}
