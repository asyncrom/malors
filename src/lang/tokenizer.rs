use std::f64::consts::*;
use crate::lang::tokenizer::State::{No, Num, Special, Word};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Name(String),
    Number(f64),
    Compare(Compare),
    Operator(Operator),
    Operation(Operation),
    Colon,
    Key(Keyword),
    ParenOpen,
    ParenClose,
    Paren(Vec<Token>)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Compare {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

impl Compare {
    pub fn compare(&self, a:f64, b:f64) -> bool {
        match *&self {
            Compare::Equal => {return a == b}
            Compare::NotEqual => {return  a != b}
            Compare::LessThan => { return a < b}
            Compare::GreaterThan => {return a > b}
            Compare::LessThanOrEqual => {return a <= b}
            Compare::GreaterThanOrEqual => {return a >= b}
        }
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Exponent,
    Log,
    Factorial,
    None,
}

impl Operator {
    pub fn operate(&self, a: f64, b: f64) -> Result<f64, String> {
        match self {
            Operator::Plus => Ok(a + b),
            Operator::Minus => Ok(a - b),
            Operator::Multiply => Ok(a * b),
            Operator::Divide => {
                if b != 0.0 {
                   Ok( a / b)
                } else {
                    Err(format!("Division by zero: {}/{}", a, b))
                }
            }
            Operator::Exponent => Ok(a.powf(b)),
            Operator::Log => Ok(a.log(b)),
            Operator::Factorial => Err("Factorials not supported for now".into()), //TODO
            Operator::None => Err("Inter: can't be none".into()) //TODO
        }
    }

    pub fn over(&self, b: Operator) -> bool {
        self.priority() >= b.priority()
    }

    fn priority(&self) -> i32 {
        match self {
            Operator::Exponent  | Operator::Log => 3,
            Operator::Multiply | Operator::Divide => 2,
            Operator::Plus | Operator::Minus => 1,
            Operator::None => 4,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Assign,
    AddVar,
    SubtractVar,
    MultiplyVar,
    DivideVar,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    If,
    While,
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    No, Word, Num, Special
}

pub(crate) fn tokenize2(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut current_string = String::new();
    let mut state = No; // Building a word

    for char in input.chars() {
        if state == Word {
            if !char.is_whitespace() && (char.is_alphanumeric() || char == '_') {
                current_string.push(char)
            } else {
                tokens.push(tokenize_name(current_string.clone()));
                current_string = String::new();
                state = No;
            }
        }
        if state == Num {
            if char.is_numeric() {
                current_string.push(char)
            } else if char == '.' {
                if current_string.contains('.') {
                    return Err("Number with two points is not possible".into());
                } else {
                    current_string.push(char);
                }
            } else {
                tokens.push(tokenize_num(current_string.clone()));
                current_string = String::new();
                state = No;
            }
        }
        if state == Special {
            if is_special(char) {
                current_string.push(char)
            } else {
                tokens.push(tokenize_special(current_string.clone())?);
                current_string = String::new();
                state = No;
            }
        }
        if state == No {
            if char.is_alphanumeric() || char == '_' {
                if char.is_alphabetic() || char == '_' {
                    current_string.push(char);
                    state = Word;
                } else if char.is_numeric() {
                    current_string.push(char);
                    state = Num;
                }
            } else if char == '(' {
                tokens.push(Token::ParenOpen);
            } else if char == ')' {
                tokens.push(Token::ParenClose);
            } else if char == ':' {
                tokens.push(Token::Colon);
            } else if is_special(char) {
                current_string.push(char);
                state = Special;
            } else if char.is_whitespace() || char.is_ascii_whitespace() {

            } else {
                return Err(format!("Incorrect char: {}", char))
            }
        }
    }

    Ok(tokens)
}

fn tokenize_name(name: String) -> Token {
        match &*name {
            "if" => Token::Key(Keyword::If),
            "wl" | "while" => Token::Key(Keyword::While),
            "del" => Token::Key(Keyword::While),
            "PI" => Token::Number(PI),
            "e" => Token::Number(E),
            _ => Token::Name(name),
        }
}

fn tokenize_num(name: String) -> Token {
    Token::Number(name.parse::<f64>().expect("Unable to convert to num"))
}

fn tokenize_special(name: String) -> Result<Token, String> {
    Ok(match &*name {
        "+" => Token::Operator(Operator::Plus),
        "-" => Token::Operator(Operator::Minus),
        "*" => Token::Operator(Operator::Multiply),
        "/" => Token::Operator(Operator::Divide),
        "**" | "^" => Token::Operator(Operator::Exponent),
        "//" => Token::Operator(Operator::Log),

        "==" => Token::Compare(Compare::Equal),
        "<" => Token::Compare(Compare::LessThan),
        "<=" => Token::Compare(Compare::LessThanOrEqual),
        ">" => Token::Compare(Compare::GreaterThan),
        ">=" => Token::Compare(Compare::GreaterThanOrEqual),

        "=" => Token::Operation(Operation::Assign),
        "+=" => Token::Operation(Operation::AddVar),
        "-=" => Token::Operation(Operation::SubtractVar),
        "*=" => Token::Operation(Operation::MultiplyVar),
        "/=" => Token::Operation(Operation::DivideVar),

        _ => return Err(format!("operation {} not supported", name))
    })
}

fn is_special(c: char) -> bool {
    c == '<' || c == '>' || c == '=' || c == '*' || c == '/' || c == '-' || c == '+' || c == '^'
}