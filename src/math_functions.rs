use itertools::Itertools;

use crate::*;

#[derive(Clone, Debug)]
struct DefinedMathFunction {
    signature : String,
    parameter_names : Vec<String>,
    expression : Expression 
}

pub fn try_reduce_parameters(expression : Expression) -> Result<Expression, String> {
    let reduced_arguments : Vec<Result<f64, String>> = expression
        .split(|token| token.is_argument_separator())
        .map(|expression| eval_expression(expression.to_vec()))
        .collect();

    if reduced_arguments.iter().any(|token| token.is_err()) { return  Err("could not reduce all of the arguments".to_string()) };

    Ok(reduced_arguments.into_iter().map(|args| Token::Number(args.unwrap())).collect::<Expression>())
}

pub fn get_arguments_from_expression(expression : Expression) -> Arguments {
    expression
        .split(|token| token.is_argument_separator())
        .map(|expression| expression.to_vec())
        .collect()
}

pub fn try_eval_builtin_math_function(signature : &str, args : Arguments) -> Result<f64, String> {
    match signature.to_lowercase().as_str() {

        // trig
        "sin" | "sine" => Ok(0.),
        "cos" | "cosine" => cos(args),
        "tan" | "tangent" =>  Ok(0.),
        "sec" | "secant" => Ok(0.),
        "csc" | "cosecant" => Ok(0.),
        "cot" | "cotangent" => Ok(0.),

        //log
        "log" => Ok(0.),

        "sum" => Ok(0.),

        // note : we might actually need a better kind of error here 
        _ => Err("err : was unable to evaluate builtin function".to_string())
    }
}

fn cos(args : Arguments) -> Result<f64, String> {
    let mut args = args;
    if args.len() != 1 { return Err("cos : expected 1 argument got 0".to_string()) }

    if let Ok(number) = eval_expression(args.pop().unwrap()) {
        Ok(number.cos())
    } else {
        Err("cos : was unable to evaluate expression argument".to_string())
    }
}