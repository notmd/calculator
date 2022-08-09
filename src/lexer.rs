use std::{fmt::Display, borrow::Borrow};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Plus,
    Dash,
    Star,
    Slash,
    Caret,
    LeftParen,
    RightParen,
    Comma,
    Number(f64),
    Function(String),
    Constant(String),
    End,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Token::Plus => "+".to_string(),
            Token::Dash => "-".to_string(),
            Token::Star => "*".to_string(),
            Token::Slash => "/".to_string(),
            Token::Caret => "^".to_string(),
            Token::LeftParen => "(".to_string(),
            Token::RightParen => ")".to_string(),
            Token::Comma => ",".to_string(),
            Token::Number(val) => val.to_string(),
            Token::Function(val) => val.to_string(),
            Token::Constant(val) => val.to_string(),
            Token::End => "".to_string(),
        };

        write!(f, "{}", c)
    }
}


pub struct Lexer {}

impl Lexer {
    pub fn lex(source: String) -> Vec<Token> {
        let mut iter = source.chars().peekable();
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(ch) = iter.next() {
            match ch {
                ' ' => continue,
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Dash),
                '*' => tokens.push(Token::Star),
                '/' => tokens.push(Token::Slash),
                '^' => tokens.push(Token::Caret),
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                'P' => {
                    if iter.peek().unwrap_or(&'a') == &'I' {
                        iter.next();
                        tokens.push(Token::Constant(String::from("PI")));
                    }
                }
                'e' => tokens.push(Token::Constant(String::from("e"))),
                ch if ch.is_ascii_digit() => {
                    let mut has_dot = false;
                    let next_numbers: String = iter
                        .clone()
                        .take_while(|c| -> bool {
                            if c.eq(&'.') && !has_dot {
                                has_dot = true;
                                iter.next();
                                return true;
                            } else if c.is_ascii_digit() {
                                iter.next();
                                return true;
                            }
                            false
                        })
                        .collect();
                    tokens.push(Token::Number(
                        format!("{}{}", ch, next_numbers).parse().unwrap(),
                    ));
                }
                ',' => tokens.push(Token::Comma),
                ch if ch.is_ascii_alphabetic() => {
                    let next_chars = iter
                        .clone()
                        .take_while(|c| -> bool {
                            if c.is_ascii_alphabetic() {
                                iter.next();
                                return true;
                            }
                            false
                        })
                        .collect::<String>();
                    let fn_name = format!("{}{}", ch, next_chars);

                    tokens.push(Token::Function(fn_name));
                }
                _ => {
                    println!("Unknown character: {}", ch)
                }
            }
        }
        tokens.push(Token::End);
        tokens
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operator {
    Add,
    Multiply,
    Divide,
    Subtract,
    Power,
    Negative,
}

impl<'a> TryFrom<Token> for Operator {
    type Error = String;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Plus => Ok(Operator::Add),
            Token::Dash => Ok(Operator::Subtract),
            Token::Star => Ok(Operator::Multiply),
            Token::Slash => Ok(Operator::Divide),
            Token::Caret => Ok(Operator::Power),
            _ => panic!("Only operator token can be convert to operator."),
        }
    }
}
