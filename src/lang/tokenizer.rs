use std::collections::VecDeque;
use crate::lang::tokenizer::Compare::{GreaterThan, LessThan};
use crate::lang::tokenizer::Operator::Multiply;
use crate::lang::tokenizer::Token::{Name, Number, Paren};

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

pub(crate) fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();

    for c in input.chars() {
        if c.is_alphanumeric() || c == '.' || c == '_' {
            current_token.push(c);
        } else {
            if !current_token.is_empty() {
                // Check if the current token starts with a number
                let mut chars = current_token.chars();
                if let Some(first_char) = chars.next() {
                    if first_char.is_numeric() {
                        let rest_of_token: String = chars.collect();
                        tokens.push(Token::Number(first_char.to_digit(10).unwrap() as f64));
                        current_token = rest_of_token;
                    } else {
                        tokens.push(tokenize_value(&current_token));
                        current_token.clear();
                    }
                }
            }

            if !c.is_whitespace() {
                let tok = tokenize_non_alphanumeric(c, &mut tokens);
                tokens.push(tok);
            }
        }
    }

    if !current_token.is_empty() {
        tokens.push(tokenize_value(&current_token));
    }

    //post_process(&mut tokens);

    tokens
}


fn tokenize_value(value: &str) -> Token {
    match value {
        "if" => Token::Key(Keyword::If),
        "while" => Token::Key(Keyword::While),
        "PI" => Token::Number(std::f64::consts::PI),
        "e" => Token::Number(std::f64::consts::E),
        _ => {
            if let Ok(number) = value.parse::<f64>() {
                Token::Number(number)
            } else {
                Token::Name(value.into())
            }
        }
    }
}

fn tokenize_non_alphanumeric(c: char, tokens: &mut Vec<Token>) -> Token {
    match c {
        ':' => Token::Colon,
        '+' | '-' | '*' | '/' | '!' | '<' | '>' | '^' | '='   => tokenize_operator(c, tokens),
        '(' => Token::ParenOpen,
        ')' => Token::ParenClose,
        _ => panic!("Unsupported character: {}", c),
    }
}

fn tokenize_operator(operator: char, tokens: &mut Vec<Token>) -> Token {
    match operator {
        '+' => Token::Operator(Operator::Plus),
        '-' => Token::Operator(Operator::Minus),
        '*' => match tokens.last() {
            Some(Token::Operator(Operator::Multiply)) => {
                tokens.pop();
                Token::Operator(Operator::Exponent)
            }
            _=> Token::Operator(Operator::Multiply)
        },
        '/' => Token::Operator(Operator::Divide),
        '^' => Token::Operator(Operator::Exponent),
        '=' => match tokens.last() {
            Some(&Token::Compare(Compare::LessThan)) => {
                tokens.pop();
                Token::Compare(Compare::LessThanOrEqual)
            }
            Some(&Token::Compare(Compare::GreaterThan)) => {
                tokens.pop();
                Token::Compare(Compare::GreaterThanOrEqual)
            }
            Some(&Token::Compare(Compare::NotEqual)) => {
                tokens.pop();
                Token::Compare(Compare::NotEqual)
            }
            Some(&Token::Operator(Operator::Plus)) => {
                tokens.pop();
                Token::Operation(Operation::AddVar)
            }
            Some(&Token::Operator(Operator::Minus)) => {
                tokens.pop();
                Token::Operation(Operation::SubtractVar)
            }
            Some(&Token::Operator(Operator::Multiply)) => {
                tokens.pop();
                Token::Operation(Operation::MultiplyVar)
            }
            Some(&Token::Operator(Operator::Divide)) => {
                tokens.pop();
                Token::Operation(Operation::DivideVar)
            }
            Some(&Token::Operation(Operation::Assign)) => {
                tokens.pop();
                Token::Compare(Compare::Equal)
            }
            _ => Token::Operation(Operation::Assign),
        },
        '!' => Token::Operator(Operator::Factorial),
        '<' => Token::Compare(LessThan),
        '>' => Token::Compare(GreaterThan),
        _ => panic!("Unsupported operator: {}", operator),
    }
}

fn post_process(tokens: &mut Vec<Token>) {

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

    for token in &mut *tokens {
        if Token::ParenOpen == *token || Token::ParenClose == *token {
            panic!("Mismatched parentheses [2]")
        }
    }
}

pub fn post_process_operation(tokens: &mut Vec<Token>) {
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

    //TODO not sure why that works
    for i in 0..tokens.len() {
        if let Token::Paren(inner_tokens) = &tokens[i] {
            let unwrapped_tokens = unwrap_nested_paren(inner_tokens.clone());
            tokens[i] = Token::Paren(unwrapped_tokens);
        }
    }


    // Add Operator::Multiply as needed
    for i in 1..tokens.len() {
        let add_operator_multiply = match (&tokens[i], &tokens[i - 1]) {
            (Token::Name(_), Token::Number(_)) => true,
            (Token::Paren(_), Token::Number(_)) | (Token::Paren(_), Token::Name(_)) => true,
            (Token::Number(_), Token::Paren(_)) | (Token::Name(_), Token::Paren(_)) => true,
            (Token::Paren(_), Token::Paren(_)) => true,
            _ => false,
        };

        if add_operator_multiply {
            tokens.insert(i, Token::Operator(Multiply));
        }
    }

}

fn unwrap_nested_paren(tokens: Vec<Token>) -> Vec<Token> {
    if tokens.len() == 1 {
        // If there's only one token inside the parentheses, unwrap it
        match &tokens[0] {
            Token::Paren(inner_tokens) => unwrap_nested_paren(inner_tokens.clone()),
            _ => tokens,
        }
    } else {
        // If there are multiple tokens inside the parentheses, return them as-is
        tokens
    }
}

fn is_valid_preceding_token(token: &Token) -> bool {
    match token {
        Token::Name(_) | Token::Number(_) | Token::ParenClose => true,
        _ => false,
    }
}
