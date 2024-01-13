use std::collections::HashMap;
use common_macros::hash_map;
use std::f64::consts;
use crate::*;

#[derive(Debug, Clone)]
pub struct BuiltInFunction {
    pub arg_count : Option<usize>,
    pub function : fn(&[f64]) -> Result<f64, String>,
}
impl BuiltInFunction {
    pub fn new(function : fn(&[f64]) -> Result<f64, String>, arg_count : Option<usize>) -> Self {
        Self {
            arg_count : arg_count,
            function : function
        }
    }

    pub fn get_built_in_functions_map() -> HashMap<&'static str, MathDefinition> {
        let built_in_function_map = hash_map! {
            "sqrt" =>           Self::new(|args| Ok(args[0].sqrt()), Some(1)),
            "cbrt" =>           Self::new(|args| Ok(args[0].cbrt()), Some(1)),
            "pow" =>            Self::new(|args| Ok(args[0].powf(args[1])), Some(2)),
            "abs" =>            Self::new(|args| Ok(args[0].abs()),  Some(1)),
            "sin" =>            Self::new(|args| Ok(args[0].sin()), Some(1)), 
            "sine" =>           Self::new(|args| Ok(args[0].sin()), Some(1)),
            "cos" =>            Self::new(|args| Ok(args[0].cos()), Some(1)),
            "cosine" =>         Self::new(|args| Ok(args[0].cos()), Some(1)),
            "tan" =>            Self::new(|args| Ok(args[0].tan()), Some(1)),
            "tangent" =>        Self::new(|args| Ok(args[0].tan()), Some(1)),
            "sec" =>            Self::new(|args| Ok(1f64 / args[0].cos()), Some(1)),
            "secant" =>         Self::new(|args| Ok(1f64 / args[0].cos()), Some(1)),
            "csc" =>            Self::new(|args| Ok(1f64 / args[0].sin()), Some(1)),
            "cosecant" =>       Self::new(|args| Ok(1f64 / args[0].sin()), Some(1)),
            "cot" =>            Self::new(|args| Ok(1f64 / args[0].tan()), Some(1)),
            "cotangent" =>      Self::new(|args| Ok(1f64 / args[0].tan()), Some(1)),
            "sinh" =>           Self::new(|args| Ok(args[0].sinh()), Some(1)),
            "cosh" =>           Self::new(|args| Ok(args[0].cosh()), Some(1)),
            "tanh" =>           Self::new(|args| Ok(args[0].tanh()), Some(1)),
            "log" =>            Self::new(|args| Ok(args[0].log10()), Some(1)),
            "log10" =>          Self::new(|args| Ok(args[0].log10()), Some(1)),
            "log2" =>           Self::new(|args| Ok(args[0].log2()), Some(1)),
            "ln" =>             Self::new(|args| Ok(args[0].ln()), Some(1)),
            "mean" =>           Self::new(|args| Ok(args.iter().fold(0f64, |acc, x| acc + x) / args.len() as f64), None),
        };
        built_in_function_map
            .into_iter()
            .map(|(identifier, built_in_function)| (identifier, MathDefinition::BuiltInFunction(built_in_function)))
            .collect()
    }
}

impl MathFunction for BuiltInFunction {
    fn evaluate(&self, args : &[f64], _math_definitions : &HashMap<&str, MathDefinition>) -> Result<f64, String> {
        if args.len() == 0 {
            return Err("error : arguments must be supplied to a function".to_string());
        }
        if self.arg_count.is_some_and(|arg_count| arg_count != args.len()) {
            return Err(format!(
                "expected {} args but recieved : {}",
                self.arg_count.unwrap(),
                args.len()
            ));
        }
        (self.function)(&args)
    }
}

pub fn get_built_in_constants_map() -> HashMap::<&'static str, MathDefinition> {
    let constants_map = hash_map! {
        "e" => consts::E,
        "pi" => consts::PI,
        "π" => consts::PI,
        "tau" => consts::TAU,
        "τ" => consts::TAU,
    };

    constants_map
        .into_iter()
        .map(|(key, value)| (key, MathDefinition::Constant(value)))
        .collect()
}