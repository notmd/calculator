use std::{iter::Peekable, rc::Rc, vec::IntoIter};

use crate::{
    constants::Constant,
    functions::{Abs, Cos, Evaluate, Max, Min, Sin, Sqrt, Tan},
    lexer::{Operator, Token},
};

#[derive(Debug, Clone)]
pub enum Expression {
    Binary(Operator, Box<Expression>, Box<Expression>),
    Unary(Operator, Box<Expression>),
    Number(f64),
    Function(Rc<dyn Evaluate>, Vec<Expression>),
    Constant(Constant),
}

impl Expression {
    pub fn eval(&self) -> f64 {
        match self {
            Expression::Number(n) => *n,
            Expression::Unary(_, expr) => -1.0 * expr.eval(),
            Expression::Binary(Operator::Add, expr1, expr2) => expr1.eval() + &expr2.eval(),
            Expression::Binary(Operator::Subtract, expr1, expr2) => expr1.eval() - expr2.eval(),
            Expression::Binary(Operator::Multiply, expr1, expr2) => expr1.eval() * expr2.eval(),
            Expression::Binary(Operator::Divide, expr1, expr2) => expr1.eval() / expr2.eval(),
            Expression::Binary(Operator::Power, expr1, expr2) => expr1.eval().powf(expr2.eval()),
            Expression::Function(f, exprs) => f.eval(exprs.to_vec()),
            Expression::Constant(c) => c.eval(),
            _ => {
                unreachable!("expression evaluate failed {:?}", self)
            }
        }
    }
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Peekable<IntoIter<Token>>) -> Self {
        Self { tokens }
    }

    pub fn parse(&mut self) -> Result<Expression, String> {
        let res = self.parse_expression();
        self.assert_next(Token::End)?;
        res
    }

    fn assert_next(&mut self, token: Token) -> Result<(), String> {
        let next = self.tokens.next();
        if let None = next {
            return Err(format!("expect next is: {:?}. Found {:?}", token, next));
        }

        if next.as_ref().unwrap() != &token {
            return Err(format!("expect next is: {:?}. Found {:?}", token, next));
        }

        Ok(())
    }

    fn parse_args(&mut self) -> Result<Vec<Expression>, String> {
        self.assert_next(Token::LeftParen)?;
        let mut exprs: Vec<Expression> = Vec::new();
        while let Some(next) = self.tokens.peek() {
            match next {
                Token::RightParen => {
                    self.tokens.next();
                    break;
                }
                Token::Comma => {
                    self.tokens.next();
                }
                _ => {
                    exprs.push(self.parse_expression()?);
                }
            }
        }

        Ok(exprs)
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        let next = self.tokens.next().unwrap();
        match next {
            Token::Number(n) => Ok(Expression::Number(n)),
            Token::Constant(name) => match name.as_str() {
                "PI" => Ok(Expression::Constant(Constant::PI)),
                "e" => Ok(Expression::Constant(Constant::E)),
                _ => Err(format!("Unknown constant: {}", name)),
            },
            Token::LeftParen => {
                let expression = self.parse_expression()?;
                self.assert_next(Token::RightParen)?;
                Ok(expression)
            }
            Token::Dash => Ok(Expression::Unary(
                Operator::Negative,
                Box::new(self.parse_term()?),
            )),
            Token::Function(name) => {
                let args = self.parse_args()?;

                let function: Result<Rc<dyn Evaluate>, ()> = match name.to_lowercase().as_str() {
                    "sin" => Ok(Rc::new(Sin {})),
                    "cos" => Ok(Rc::new(Cos {})),
                    "tan" => Ok(Rc::new(Tan {})),
                    "sqrt" => Ok(Rc::new(Sqrt {})),
                    "abs" => Ok(Rc::new(Abs {})),
                    "max" => Ok(Rc::new(Max {})),
                    "min" => Ok(Rc::new(Min {})),
                    _ => Err(()),
                };

                match function {
                    Ok(f) => Ok(Expression::Function(f, args)),
                    Err(_) => Err(format!("Unknown function: {}", name)),
                }
            }
            _ => {
                Err(format!("Unexpected token: {}", next))
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_term()?;
        while let Some(next) = self.tokens.clone().peek() {
            match next {
                Token::Plus | Token::Dash => {
                    self.tokens.next();
                    let expr2 = self.parse_term()?;
                    expr = Expression::Binary(
                        Operator::try_from(next.clone()).unwrap(),
                        Box::new(expr),
                        Box::new(expr2),
                    );
                }
                _ => {
                    break;
                }
            }
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_primary()?;
        let next = self.tokens.peek().unwrap();
        if *next == Token::Caret {
            self.tokens.next();
            let expression2 = self.parse_factor()?;
            expr = Expression::Binary(Operator::Power, Box::new(expr), Box::new(expression2));
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expression, String> {
        let mut expression = self.parse_factor()?;
        while let Some(next) = self.tokens.clone().peek() {
            match next {
                Token::Star | Token::Slash => {
                    self.tokens.next();
                    let expression2 = self.parse_factor()?;
                    expression = Expression::Binary(
                        Operator::try_from(next.clone()).unwrap(),
                        Box::new(expression),
                        Box::new(expression2),
                    );
                }
                _ => {
                    break;
                }
            }
        }

        Ok(expression)
    }
}
