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
        '<' => Token::Compare(Compare::LessThan),
        '>' => Token::Compare(Compare::GreaterThan),
        _ => panic!("Unsupported operator: {}", operator),
    }
}
