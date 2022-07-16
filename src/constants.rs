use std::f64::consts::{PI, E};

#[derive(Debug, Clone)]
pub enum Constant {
    PI,
    E,
}

impl Constant {
    pub fn eval(&self) -> f64 {
        match self {
            Constant::PI => PI,
            Constant::E => E,
        }
    }
}
