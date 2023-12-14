use std::collections::HashMap;
use crate::lang::runner::Possible::{PossExpression, PossToken};
use crate::lang::tokenizer::{Operation, Operator, Token, tokenize};
use crate::lang::tokenizer::Token::{Name, Number};

pub fn run_line(memory: &mut HashMap<&str, f64>, tokens: Vec<Token>) {
    //TODO implement functions
    if tokens.is_empty() {
        return;
    }

    ;
}

fn var(memory: &mut HashMap<&str, f64>, tokens: Vec<Token>) {

}

fn three_composer(tokens: Vec<Token>) {
    //TODO Use Constructor to compose three from top to bottom, then transform into Possible tree !
}

pub struct Constructor {
    a: Vec<Token>,
    o: Operator,
    b: Vec<Token>,
}
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