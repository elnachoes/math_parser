use common_macros::hash_map;
use std::{collections::HashMap, f64::consts};
use crate::{MathDefinition, Expression, Token, Operator};

pub struct Constant {
    signature : String,
    value : f64
}
impl Constant {
    pub fn parse(expression : &Expression) -> Result<Self, String> {
        if let [Token::Identity(identity), Token::ConstantAssignment, Token::Number(num)] = expression.as_slice() {
            Ok(Self {
                signature : identity.to_string(),
                value : *num
            })
        } else {
            Err("could not parse constant from expression. constants must be defined as so : lethal_gravity = 98.1".to_string())
        }
    }

    pub fn value(&self) -> f64 { self.value }
    
    pub fn signature(&self) -> &String { &self.signature }

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
}
