use std::collections::HashMap;
use crate::lang::calculator::calculate;
use crate::lang::line_type::{construct_line_type, LineType};
use crate::lang::LineResult;
use crate::lang::tokenizer::{Operation, Token};
use crate::lang::tokenizer::Token::{Number, Paren};

pub fn run(memory: &mut HashMap<String, f64>, line_type: LineType) -> Result<LineResult, String> {
    match line_type {
        LineType::Nothing => {

        }
        LineType::Out(out) => {
            let mut result = "".to_string();
            for string in out {
                result.push_str(":");
                let num = get_from_mem(memory, string)?;
                result.push_str(&*format!("{}", num));
            }
            return Ok(LineResult::Output(result))
        }
        LineType::VarOperate(var, op, ex) => {
            let num = result(memory,ex)?;
            match op {
                Operation::Assign => {
                    memory.insert(var, num);
                }
                Operation::AddVar => {
                    let ancient = get_from_mem(memory, var.clone())?;
                    memory.insert(var, ancient + num);
                }
                Operation::SubtractVar => {
                    let ancient = get_from_mem(memory, var.clone())?;
                    memory.insert(var, ancient - num);
                }
                Operation::MultiplyVar => {
                    let ancient = get_from_mem(memory, var.clone())?;
                    memory.insert(var, ancient * num);
                }
                Operation::DivideVar => {
                    let ancient = get_from_mem(memory, var.clone())?;
                    memory.insert(var, ancient / num);
                }
            }
        }
        LineType::If(a, c, b, actions) => {
            let a = result(memory, a)?;
            let b = result(memory, b)?;
            if c.compare(a, b) {
                for action in actions {
                    let lt = construct_line_type(action)?;
                    let result = run(memory, lt);
                    if let Ok(LineResult::Output(res)) = result {
                        println!("{}", res)
                    } else if let Err(err) = result {
                        println!("PROGRAM ERROR:\n{}", err);
                        println!("Instruction skipped");
                    }
                }
            }
            return Ok(LineResult::Nothing);
        }
        LineType::While(ref a, ref c, ref b, ref actions) => {
            let mut line_types = Vec::new();
            for action in actions {
                line_types.push(construct_line_type(action.clone())?)
            }
            while c.compare(result(memory, a.clone())?, result(memory, b.clone())?) == true {
                for lt in &line_types {
                    //let lt = construct_line_type(action.clone())?;
                    let result = run(memory, lt.clone());
                    if let Ok(LineResult::Output(res)) = result {
                        println!("{}", res)
                    } else if let Err(err) = result {
                        println!("PROGRAM ERROR:\n{}", err);
                        println!("Instruction skipped");
                    }
                }
            }

            return Ok(LineResult::Nothing);
        }
    }
    Ok(LineResult::Nothing)
}

fn replace_var(memory: &mut HashMap<String, f64>, tokens : Vec<Token>) -> Result<Vec<Token>, String> {
    let mut tokens = tokens;
    for i in 0..tokens.len() {
        if let Token::Name(name) = tokens.get(i).unwrap() {
            if let Some(num) = memory.get(name) {
                tokens[i] = Number(*num)
            } else if let None = memory.get(name) {
                return Err(format!("Var [{}] doesn't exist", name))
            } {

            }
        } else if let Paren(toks) = tokens.get(i).unwrap() {
            tokens[i] = Paren(replace_var(memory, toks.clone())?)
        }
    }
    return Ok(tokens);
}

fn result(memory: &mut HashMap<String, f64>, tokens : Vec<Token>) -> Result<f64, String> {
    let rep = replace_var(memory,tokens)?;
    let cal = calculate(rep)?;
    return Ok(cal);
}

pub fn get_from_mem(memory: &mut HashMap<String, f64>, string: String) -> Result<f64, String> {
    return match memory.get(&string) {
        None => {
            Err(format!("Var [{}] not found in memory", string))
        }
        Some(num) => {
            Ok(*num)
        }
    }
}
