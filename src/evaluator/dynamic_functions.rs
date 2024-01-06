use crate::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct DynamicMathFunction {
    pub signature: Option<String>,
    pub arg_names: Vec<String>,
    pub expression: Expression,
}
impl DynamicMathFunction {
    pub fn anonymous(arg_names: Vec<String>, expression: Expression) -> Result<Self, String> {
        Ok(Self {
            signature: None,
            arg_names: arg_names,
            expression: expression,
        })
    }

    //todo
    pub fn validate_function() -> Result<(), String> {
        Ok(())
    }

    // todo
    pub fn evaluate(&self, args: &[f64]) -> Result<f64, String> {
        if args.len() == 0 {
            return Err("error : arguments must be supplied to a function".to_string());
        }

        if args.len() != self.arg_names.len() {
            return Err(format!(
                "expected {} args but recieved : {}",
                self.arg_names.len(),
                args.len()
            ));
        }

        let variable_arg_map = self
            .arg_names
            .iter()
            .enumerate()
            .map(|(index, arg_name)| (arg_name, args[index]))
            .collect::<HashMap<&String, f64>>();

        let expression_to_evaluate = self
            .expression
            .clone()
            .into_iter()
            .map(|token| {
                if let Token::Identity(identity) = token {
                    Token::Number(variable_arg_map[&identity])
                } else {
                    token
                }
            })
            .collect();

        eval_expression(expression_to_evaluate, true)
    }
}
