use std::fmt::Debug;

use crate::Expression;

pub trait Evaluate: Debug {
    // TODO Vec<Expression> should be Arguments and return type should be Result<f64, String or something>
    fn eval(&self, expr: Vec<Expression>) -> f64;
}

#[derive(Debug)]
pub struct Sin {}

impl Evaluate for Sin {
    fn eval(&self, exprs: Vec<Expression>) -> f64 {
        assert_eq!(exprs.len(), 1);
        exprs[0].eval().sin()
    }
}
#[derive(Debug)]
pub struct Cos {}

impl Evaluate for Cos {
    fn eval(&self, exprs: Vec<Expression>) -> f64 {
        assert_eq!(exprs.len(), 1);
        exprs[0].eval().cos()
    }
}
