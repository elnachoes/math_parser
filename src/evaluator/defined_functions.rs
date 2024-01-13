use itertools::Itertools;

use crate::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct DefinedFunction {
    pub signature: Option<String>,
    pub arg_names: Vec<String>,
    pub expression: Expression,
}
impl DefinedFunction {
    pub fn anonymous(arg_names: Vec<String>, expression: Expression) -> Result<Self, String> {
        Ok(Self {
            signature: None,
            arg_names: arg_names,
            expression: expression,
        })
    }

    pub fn parse_str(str: &str) -> Result<Self, String> {
        Self::parse(parse_str(str)?)
    }

    pub fn parse(expression: Expression) -> Result<Self, String> {
        let mut expression = expression;

        // parse out the signature of the function
        let signature = if let Token::Identity(signature) = expression.remove(0) {
            signature
        } else {
            return Err("missing signature".to_string());
        };

        if let Token::Operator(Operator::OpenParen) = expression.remove(0) {
        } else {
            return Err("expected open bracket".to_string());
        }

        let index_of_closing_brace = expression
            .iter()
            .enumerate()
            .find_or_first(|(_index, token)| token.is_close_paren())
            .map(|(index, _token)| index)
            .ok_or("expected close bracket".to_string())?;

        let argument_names_tokens = expression
            .drain(0..index_of_closing_brace)
            .collect::<Expression>();

        if argument_names_tokens.iter().any(|token| {
            if let Token::Identity(_) | Token::Operator(Operator::ArgumentSeparator) = token {
                false
            } else {
                true
            }
        }) {
            return Err("expected identity or argument seperator".to_string());
        }

        let mut argument_names: Vec<String> = vec![];
        let mut expected_token_type: Token = Token::Identity(String::default());
        for token in argument_names_tokens.into_iter() {
            match token {
                Token::Identity(identity) if expected_token_type.is_identity() => {
                    argument_names.push(identity);
                    expected_token_type = Token::Operator(Operator::ArgumentSeparator)
                }
                Token::Operator(Operator::ArgumentSeparator)
                    if expected_token_type.is_argument_separator() =>
                {
                    expected_token_type = Token::Identity(String::default())
                }
                _ => return Err(format!("expected {expected_token_type:?}")),
            }
        }

        if let &[Token::Operator(Operator::CloseParen), Token::Operator(Operator::FunctionAssignment)] =
            expression.drain(0..2).as_slice()
        {
        } else {
            return Err("expected close paren and function assigment operator".to_string());
        }

        let defined_function = Self {
            arg_names: argument_names,
            expression: expression,
            signature: Some(signature),
        };

        if !defined_function.valid() {
            Err("invalid function, undeclared variable used".to_string())
        } else {
            Ok(defined_function)
        }
    }

    /// this will validate a dynamic function.
    /// TODO : we need to make more in depth errors here and possibly account for external variables
    /// EX if functions and variables are stored in external state but are used in a function it needs to
    /// be accounted for.
    pub fn valid(&self) -> bool {
        self.expression.iter().all(|token| {
            !token.is_identity()
                || token
                    .get_identity()
                    .is_some_and(|identity| self.arg_names.contains(identity))
        })
    }
}

impl MathFunction for DefinedFunction {
    fn evaluate(&self, args : &[f64], math_definitions : &HashMap<&str, MathDefinition>) -> Result<f64, String> {
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
                match token {
                    Token::Identity(identity) if !math_definitions.contains_key(identity.as_str()) => Token::Number(variable_arg_map[&identity]),
                    _ => token
                }
            })
            .collect();

        eval_expression(expression_to_evaluate, math_definitions, true)
    }
}