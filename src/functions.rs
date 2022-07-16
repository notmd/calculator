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
#[derive(Debug)]
pub struct Tan {}

impl Evaluate for Tan {
    fn eval(&self, exprs: Vec<Expression>) -> f64 {
        assert_eq!(exprs.len(), 1);
        exprs[0].eval().tan()
    }
}
#[derive(Debug)]
pub struct Sqrt {}

impl Evaluate for Sqrt {
    fn eval(&self, exprs: Vec<Expression>) -> f64 {
        assert_eq!(exprs.len(), 1);
        exprs[0].eval().sqrt()
    }
}
#[derive(Debug)]
pub struct Abs {}

impl Evaluate for Abs {
    fn eval(&self, exprs: Vec<Expression>) -> f64 {
        assert_eq!(exprs.len(), 1);
        let val = exprs[0].eval();

        if val > 0.0 {
            val
        } else {
            -val
        }
    }
}
#[derive(Debug)]
pub struct Max {}

impl Evaluate for Max {
    fn eval(&self, exprs: Vec<Expression>) -> f64 {
        assert!(exprs.len() > 1);
        let mut max = exprs[0].eval();

        for expr in exprs {
            let val = expr.eval();
            if val > max {
                max = val;
            }
        }
        max
    }
}
#[derive(Debug)]
pub struct Min {}

impl Evaluate for Min {
    fn eval(&self, exprs: Vec<Expression>) -> f64 {
        assert!(exprs.len() > 1);
        let mut min = exprs[0].eval();

        for expr in exprs {
            let val = expr.eval();
            if val < min {
                min = val;
            }
        }
        min
    }
}
