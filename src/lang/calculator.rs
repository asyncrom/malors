/*
The calculator takes an expression only made of Num(num) and Operator and it returns the result as an f64
 */

use crate::lang::calculator::Possible::{PossExpression, PossToken};
use crate::lang::tokenizer::{Operator, Token};
use crate::lang::tokenizer::Operator::Multiply;
use crate::lang::tokenizer::Token::{Name, Number};

pub fn calculate(tokens: Vec<Token>) -> f64 {
    // Post process the tokens to add implicit multiplications, remove unnecessary parenthesis and determine minus signs
    let tokens = post_process_operation(tokens);
    println!("post pro {:?}", tokens);
    // Compose a three with branch A and branch B possibles and the node an operation
    // A possible is either an Expression that needs to be resolved or a value
    let three = three_composer(tokens);

    // Resolve the three by propagating the solve methode
    // If the expression given was correct, it shouldn't be a PossToken
    let mut result = None;
    if let Possible::PossExpression(expression) = three {
        result = Some(expression.solve());
    } else if let Possible::PossToken(tok) = three {
        if let Number(num) = tok {
            result = Some(num)
        } else {
            panic!("Invalid token [27]")
        }
    }
    result.expect("Invalide result in calculate")
}
/*

Post process functions

 */
fn post_process_operation(tokens: Vec<Token>) -> Vec<Token> {
    let mut tokens = tokens;
    // Post process minus signs
    let mut index = 0;
    while index < tokens.len() {
        if let Token::Operator(Operator::Minus) = &tokens[index] {
            if index == 0 || !is_valid_preceding_token(&tokens[index - 1]) {
                if let Some(Token::Number(number)) = tokens.get(index + 1) {
                    tokens[index] = Token::Number(-number.clone());
                    tokens.remove(index + 1);
                }
            }
        }

        index += 1;
    }

    // Post process parenthesis
    let mut open = Vec::new();
    let mut index_adjustment = 0;  // Track the adjustment in indices due to removals and insertions

    for i in 0..tokens.len() {
        let adjusted_index = i - index_adjustment;
        let token = tokens[adjusted_index].clone();

        if let Token::ParenOpen = token {
            open.push(adjusted_index);
        } else if let Token::ParenClose = token {
            let start = *open.last().expect("Mismatched parentheses [1]");
            let end = adjusted_index;
            let mut extracted: Vec<Token> = tokens.drain(start..=end).collect();

            // Adjust indices for the removals
            index_adjustment += extracted.len() - 1;

            // Remove the first and last elements (parentheses)
            extracted.pop();
            extracted.remove(0);

            // Insert the Paren token at the original start index
            tokens.insert(start, Token::Paren(extracted));

            open.pop();
        }
    }
    // Check parentheses not in pair, in this case, throw an error
    for token in &mut *tokens {
        if Token::ParenOpen == *token || Token::ParenClose == *token {
            panic!("Mismatched parentheses [2]")
        }
    }

    // Add Operator::Multiply as needed
    for i in 1..tokens.len() {
        let add_operator_multiply = match (&tokens[i], &tokens[i - 1]) {
            (Token::Name(_), Token::Number(_)) => true,
            (Token::Name(_), Token::Name(_)) => true,
            (Token::Number(_), Token::Number(_)) => true,
            (Token::Paren(_), Token::Number(_)) | (Token::Paren(_), Token::Name(_)) => true,
            (Token::Number(_), Token::Paren(_)) | (Token::Name(_), Token::Paren(_)) => true,
            (Token::Paren(_), Token::Paren(_)) => true,
            _ => false,
        };

        if add_operator_multiply {
            tokens.insert(i, Token::Operator(Multiply));
        }
    }
    //TODO not sure why that works
    //Unwrap nested parentheses recursively
    let mut i = 0;
    while i < tokens.len() {
        if let Token::Paren(inner_tokens) = &tokens[i] {
            if inner_tokens.len() == 1 {
                // Unwrap the nested parentheses
                let nested_inner_tokens = inner_tokens[0].clone();
                tokens[i] = nested_inner_tokens;
                // Continue processing at the same index to handle multiple levels of nesting
                continue;
            }
        }
        i += 1;
    }

    // for i in 0..tokens.len() {
    //     let current = tokens.get(i).unwrap();
    //     if let Token::Paren(inner_tokens) = current {
    //         tokens[i] = remove_paren(current.clone());
    //     }
    // }

    tokens
}

fn remove_paren(token: Token) -> Token {
    if let Token::Paren(ref inner_tokens) = token {
        if inner_tokens.len() == 1 {
            if let Token::Paren(_) = inner_tokens.get(0).unwrap() {
                return remove_paren(inner_tokens.get(0).unwrap().clone())
            } else {
                return inner_tokens.get(0).unwrap().clone()
            }
        } else {
            return token.clone()
        }
    } else {
        panic!("Unexpected Error [137]")
    }
}
fn is_valid_preceding_token(token: &Token) -> bool {
    match token {
        Token::Name(_) | Token::Number(_) | Token::ParenClose => true,
        _ => false,
    }
}
/*

Three creation functions

 */
#[derive(Debug)]
pub struct Expression {
    a: Box<Possible>,
    o: Operator,
    b: Box<Possible>,
}

impl Expression {
    pub fn new(a: Possible, o:Operator, b: Possible) -> Expression {
        Expression {a: Box::from(a), o, b: Box::from(b)}
    }
    pub fn solve(&self) -> f64 {
        let a:f64 = match &*self.a {
            PossExpression(expr) => expr.solve(),
            PossToken(tok) => {
                match tok {
                    Number(num) => *num,
                    _ => panic!("Invalid token in expression")
                }
            }
        };
        let b:f64 = match &*self.b {
            PossExpression(expr) => expr.solve(),
            PossToken(tok) => {
                match tok {
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
    PossToken(Token),
}

impl Possible {
    pub fn expression(exp: Expression) -> Possible {
        PossExpression(Box::from(exp))
    }
    pub fn token(tok: Token) -> Possible {
        PossToken(tok)
    }
}

fn three_composer(tokens: Vec<Token>) -> Possible {
    if tokens.len() == 1 {
        return Possible::token(tokens.get(0).unwrap().clone())
    }
    let mut version: (Vec<Token>, Operator, Vec<Token>) = (vec![], Operator::None, vec![]);
    for i in 0..tokens.len() {
        if i > 0 && i < tokens.len() - 1 {
            let token = tokens[i].clone();
            if let Token::Operator(o) = token {
                if version.1.clone().over(o.clone()) {
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


    //println!("version: {:?}", version);
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