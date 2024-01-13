use std::collections::HashMap;
use crate::MathDefinition;

pub trait MathFunction {
    fn evaluate(&self, args : &[f64], math_definitions : &HashMap<&str, MathDefinition>) -> Result<f64, String>;
}