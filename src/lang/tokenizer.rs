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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Exponent,
    Factorial,
    None,
}

impl Operator {
    pub fn operate(&self, a: f64, b: f64) -> f64 {
        match self {
            Operator::Plus => a + b,
            Operator::Minus => a - b,
            Operator::Multiply => a * b,
            Operator::Divide => {
                if b != 0.0 {
                    a / b
                } else {
                    panic!("Division by zero");
                }
            }
            Operator::Exponent => a.powf(b),
            Operator::Factorial => panic!("Factorials not supported"), //TODO
            Operator::None => panic!("Can't be none") //TODO
        }
    }

    pub fn over(&self, b: Operator) -> bool {
        self.priority() >= b.priority()
    }

    fn priority(&self) -> i32 {
        match self {
            Operator::Exponent => 3,
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

pub(crate) fn tokenize2(input: &str) -> Vec<Token> {
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
                    panic!("Number with two points is not possible")
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
                tokens.push(tokenize_special(current_string.clone()));
                current_string = String::new();
                state = No;
            }
        }
        if state == No {
            if char.is_alphanumeric() {
                if char.is_alphabetic() {
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
                panic!("Incorrect char: {}", char)
            }
        }
    }

    tokens
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

fn tokenize_special(name: String) -> Token {
    match &*name {
        "+" => Token::Operator(Operator::Plus),
        "-" => Token::Operator(Operator::Minus),
        "*" => Token::Operator(Operator::Multiply),
        "/" => Token::Operator(Operator::Divide),
        "**" | "^" => Token::Operator(Operator::Exponent),

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

        _ => panic!("operation {} not supported", name)
    }
}

fn is_special(c: char) -> bool {
    c == '<' || c == '>' || c == '=' || c == '*' || c == '/' || c == '-' || c == '+' || c == '^'
}