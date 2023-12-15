use std::collections::HashMap;
use std::thread;
use crate::lang::runner::Possible::{PossExpression, PossToken};
use crate::lang::tokenizer::{Operation, Operator, post_process_operation, Token, tokenize};
use crate::lang::tokenizer::Token::{Name, Number};

pub fn compile_line(memory: &mut HashMap<&str, f64>, tokens: Vec<Token>) {
    //TODO implement functions
    let mut tokens = tokens;
    post_process_operation(&mut tokens);
    let three = three_composer(tokens);
    println!("three: {:?}", three);
}

fn var(memory: &mut HashMap<&str, f64>, tokens: Vec<Token>) {

}

fn three_composer(tokens: Vec<Token>) -> Possible {
    let mut version: (Vec<Token>, Operator, Vec<Token>) = (vec![], Operator::None, vec![]);
    for i in 0..tokens.len() {
      if i > 0 && i < tokens.len() - 1 {
          let token = tokens[i].clone();
          if let Token::Operator(o) = token {
              if o.over(version.1.clone()) {
                  println!("here");
                  let tokens_copy = tokens.clone();
                  let (mut a, mut b) = tokens_copy.split_at(i);
                  let a = Vec::from(a);
                  let b: Vec<Token> = Vec::from(&b[1..]);
                  version = (a, o, b);
              }
          }
      }
    }

    println!("version: {:?}", version);
    let a =
        if version.0.len() == 1 {
            if let Some(Token::Paren(toks)) = version.0.get(0) {
                if toks.len() == 1 {
                    Possible::token(toks.get(0).unwrap().clone())
                } else {
                    three_composer(toks.clone())
                }
            } else {
                Possible::token(version.0.get(0).unwrap().clone())
            }
        } else {
            three_composer(version.0)
        };
    let b =
        if version.2.len() == 1 {
            if let Some(Token::Paren(toks)) = version.2.get(0) {
                three_composer(toks.clone())
            } else {
                Possible::token(version.2.get(0).unwrap().clone())
            }
        } else {
            three_composer(version.2)
        };
    Possible::expression(Expression::new(a, version.1, b))
}
#[derive(Debug)]
pub struct Expression {
    a: Box<Possible>,
    o: Operator,
    b: Box<Possible>,
}

impl Expression {
    pub fn new(a: Possible, o:Operator, b:Possible) -> Expression {
        Expression {a: Box::from(a), o, b: Box::from(b)}
    }
    pub fn solve(&self, mem: &HashMap<String, f64>) -> f64 {
        let a:f64 = match &*self.a {
            PossExpression(expr) => expr.solve(mem),
            PossToken(tok) => {
                match tok {
                    Name(name) => *mem.get(name).expect(&*format!("Can't find {} in memory", name)),
                    Number(num) => *num,
                    _ => panic!("Invalid token in expression")
                }
            }
        };
        let b:f64 = match &*self.b {
            PossExpression(expr) => expr.solve(mem),
            PossToken(tok) => {
                match tok {
                    Name(name) => *mem.get(name).expect(&*format!("Can't find {} in memory", name)),
                    Number(num) => *num,
                    _ => panic!("Invalid token in expression")
                }
            }
        };

        let result: f64 = self.o.operate(a, b);
        return result;
    }
}

#[derive(Debug)]
pub enum Possible {
    PossExpression(Box<Expression>),
    PossToken(Token)
}

impl Possible {
    pub fn expression(exp: Expression) -> Possible {
        PossExpression(Box::from(exp))
    }
    pub fn token(tok: Token) -> Possible {
        PossToken(tok)
    }
}