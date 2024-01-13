use crate::{DefinedFunction, BuiltInFunction, get_built_in_constants_map};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum MathDefinition {
    Constant(f64),
    BuiltInFunction(BuiltInFunction),
    DefinedFunction(DefinedFunction)
} 
impl MathDefinition {
    pub fn default_math_definitions<'a>() -> HashMap<&'a str, MathDefinition> {
        let mut math_state = HashMap::new();
        math_state.extend(BuiltInFunction::get_built_in_functions_map());
        math_state.extend(get_built_in_constants_map());
        math_state
    }

    pub fn is_constant(&self) -> bool {
        if let Self::Constant(_) = self {
            true
        } else {
            false
        }
    }

    pub fn get_constant(&self) -> Option<f64> {
        if let Self::Constant(constant) = self {
            Some(*constant)
        } else {
            None
        }
    } 
}
