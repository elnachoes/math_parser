use std::{borrow::Borrow, collections::HashMap, error, fmt::Arguments, ops::RangeInclusive};

use itertools::Itertools;

use crate::*;

enum NumOpNumDirection {
    LeftToRight,
    RightToLeft,
}
impl NumOpNumDirection {
    pub fn get_adjacent_operand_index(&self, index: usize) -> usize {
        match self {
            Self::LeftToRight => index + 2,
            Self::RightToLeft => index - 2,
        }
    }

    pub fn get_operation_range_at_index(&self, index: usize) -> RangeInclusive<usize> {
        match self {
            Self::LeftToRight => index..=index + 2,
            Self::RightToLeft => index - 2..=index,
        }
    }

    pub fn index_at_expression_end(&self, tokens: &Expression, index: usize) -> bool {
        match self {
            Self::LeftToRight => index == tokens.len() - 1,
            Self::RightToLeft => index == 0,
        }
    }

    pub fn get_starting_index(&self, tokens: &Expression) -> Result<usize, String> {
        if tokens.len() == 0 {
            return Err("error : no starting index for token string of length 0".to_string());
        }
        match self {
            Self::LeftToRight => Ok(0),
            Self::RightToLeft => Ok(tokens.len() - 1),
        }
    }
}

fn eval_num_op_num_operators(
    tokens: Expression,
    operators: &[Operator],
    eval_direction: NumOpNumDirection,
) -> Result<Expression, String> {
    let mut tokens = tokens;

    let mut index = eval_direction.get_starting_index(&tokens)?;

    loop {
        if eval_direction.index_at_expression_end(&tokens, index) || tokens.len() == 1 {
            break;
        }

        let operation_range = eval_direction.get_operation_range_at_index(index);

        let reduction_option = if let &[Token::Number(left_num), Token::Operator(op), Token::Number(right_num)] =
            &tokens[operation_range.clone()]
        {
            if operators.iter().contains(&op) {
                op.apply_operation(left_num, right_num)
            } else {
                Err(())
            }
        } else {
            Err(())
        };

        if let Ok(result) = reduction_option {
            let insertion_index = *operation_range.start();
            tokens.drain(operation_range.clone());
            let new_number_token = Token::Number(result);
            if tokens.len() == 0 {
                tokens.push(new_number_token)
            } else {
                tokens.insert(insertion_index, new_number_token)
            }
            if let NumOpNumDirection::RightToLeft = eval_direction {
                index -= 2
            }
        } else {
            index = eval_direction.get_adjacent_operand_index(index);
        }
    }
    Ok(tokens)
}



// todo : build a reduce addition subtraction operators


// todo : build a eval single operator single operand for things like factorial or a negative sign present by itself next to a number
// fn reduce_addition_subtraction_operators()


/// this will find the end of a sub expression by itterating through the token string and finding where the scope is enclosed.
fn find_sub_expression_end(
    tokens: &Expression,
    expression_start_index: usize,
) -> Result<usize, String> {
    if expression_start_index >= tokens.len() || tokens.get(expression_start_index).is_none() {
        return Err("error : find_expression_end".to_string());
    }

    if expression_start_index == 0 {
        return Ok(tokens.len() - 1);
    }

    let mut current_expression_scope = 0;
    for (i, token) in tokens.iter().enumerate().skip(expression_start_index) {
        if token.is_open_paren() {
            current_expression_scope += 1
        } else if token.is_close_paren() {
            if current_expression_scope == 0 {
                return Ok(i - 1);
            }
            if current_expression_scope != 0 {
                current_expression_scope -= 1
            }
        }
    }

    Err("find_sub_expression_end : could not find sub expression end".to_string())
}

/// this will check if a token string is solved. if there is only one number token left in the token string the expression is solved.
fn is_solved_token_string(tokens: &Expression) -> bool {
    if tokens.len() == 1 && tokens.first().is_some_and(|token| token.is_num()) {
        true
    } else {
        false
    }
}

/// this will evaluate a string math expression
pub fn eval_str(string: &str) -> Result<f64, String> {
    let token_string = parse_str(string)?;
    eval_expression(token_string)
}

/// this will evaluate a token string expresssion recursively.
pub fn eval_expression(tokens: Expression) -> Result<f64, String> {
    // if the token string comes in solved return the solved answer
    if is_solved_token_string(&tokens) {
        return Ok(tokens.first().unwrap().to_f64().unwrap());
    }

    // find each sub expression and store a list of the answer and range of tokens they will replace.
    let mut sub_expression_solutions: Vec<(Token, RangeInclusive<usize>)> = vec![];
    for (index, _token) in tokens
        .iter()
        .enumerate()
        .filter(|(_index, token)| token.is_open_paren())
    {
        if sub_expression_solutions
            .iter()
            .any(|(_token, range)| range.contains(&index))
        {
            continue;
        }

        // todo : figure out if there is a sub expression OR a function that needs to get solved.
        let pre_calc_start_index = index + 1;
        let pre_calc_end_index = find_sub_expression_end(&tokens, pre_calc_start_index)?;

        // if the prior token to the open paren is an identity, a function is being invoked and must be solved, otherwise solve a sub expression.
        let sub_expression_result_range = if index != 0
            && tokens
                .iter()
                .nth(index - 1)
                .is_some_and(|token| token.is_identity())
        {
            let function_signature = tokens
                .iter()
                .nth(index - 1)
                .unwrap()
                .identity_string()
                .unwrap();
            let function_result = try_eval_builtin_math_function(
                function_signature,
                tokens[pre_calc_start_index..=pre_calc_end_index].to_vec(),
            )?;
            (
                Token::Number(function_result),
                index - 1..=pre_calc_end_index + 1,
            )
        } else {
            let sub_expression_result =
                eval_expression(tokens[pre_calc_start_index..=pre_calc_end_index].to_vec())?;
            (
                Token::Number(sub_expression_result),
                index..=pre_calc_end_index + 1,
            )
        };

        sub_expression_solutions.push(sub_expression_result_range)
    }

    // replace each token in the string with
    let mut after_sub_expressions = tokens;
    let mut reduction_offset = 0;
    for (token, range) in sub_expression_solutions {
        let offset_range = range.start() - reduction_offset..=range.end() - reduction_offset;
        after_sub_expressions.drain(offset_range.clone());
        reduction_offset += offset_range.clone().count() - 1;
        after_sub_expressions.insert(*offset_range.start(), token);
    }
    if is_solved_token_string(&after_sub_expressions) {
        return Ok(after_sub_expressions.first().unwrap().to_f64().unwrap());
    }

    let after_exp = eval_num_op_num_operators(
        after_sub_expressions,
        &[Operator::Exponentiation],
        NumOpNumDirection::RightToLeft,
    )?;
    if is_solved_token_string(&after_exp) {
        return Ok(after_exp.first().unwrap().to_f64().unwrap());
    }

    let after_mult_div = eval_num_op_num_operators(
        after_exp,
        &[
            Operator::Multiplication,
            Operator::Division,
            Operator::Modulus,
        ],
        NumOpNumDirection::LeftToRight,
    )?;
    if is_solved_token_string(&after_mult_div) {
        return Ok(after_mult_div.first().unwrap().to_f64().unwrap());
    }

    // todo : we need to reduce addition subtraction operators here prior to evaluating them

    let after_add_sub = eval_num_op_num_operators(
        after_mult_div,
        &[Operator::Addition, Operator::Subtraction],
        NumOpNumDirection::LeftToRight,
    )?;
    if is_solved_token_string(&after_add_sub) {
        return Ok(after_add_sub.first().unwrap().to_f64().unwrap());
    }
    
    Err("error : unsolved expression : {}".to_string())
}

fn try_reduce_args(expression: Expression) -> Result<Expression, String> {
    println!("trying to reduce this expression : {expression:?}");
    let reduced_arguments: Vec<Result<f64, String>> = expression
        .split(|token| token.is_argument_separator())
        .map(|expression| {
            println!("expression to reduce -> {expression:?}");
            eval_expression(expression.to_vec())
        })
        .collect();

    if reduced_arguments.iter().any(|token| token.is_err()) {
        return Err("could not reduce all of the arguments".to_string());
    };

    Ok(reduced_arguments
        .into_iter()
        .map(|args| Token::Number(args.unwrap()))
        .collect::<Expression>())
}

/// this maps built in functions to signatures for trig/alc/calc/stats/etc functions
fn try_eval_builtin_math_function(signature: &str, args: Expression) -> Result<f64, String> {
    match signature {
        // alg
        "sqrt" => evaluate_static_math_func(|args| Ok(args[0].sqrt()), args, Some(1)),
        "cbrt" => evaluate_static_math_func(|args| Ok(args[0].cbrt()), args, Some(1)),
        "pow" => evaluate_static_math_func(|args| Ok(args[0].powf(args[1])), args, Some(2)),
        "abs" => evaluate_static_math_func(|args| Ok(args[0].abs()), args, Some(1)),

        // trig
        "sin" | "sine" => evaluate_static_math_func(|args| Ok(args[0].sin()), args, Some(1)),
        "cos" | "cosine" => evaluate_static_math_func(|args| Ok(args[0].cos()), args, Some(1)),
        "tan" | "tangent" => evaluate_static_math_func(|args| Ok(args[0].tan()), args, Some(1)),
        "sec" | "secant" => {
            evaluate_static_math_func(|args| Ok(1f64 / args[0].cos()), args, Some(1))
        }
        "csc" | "cosecant" => {
            evaluate_static_math_func(|args| Ok(1f64 / args[0].sin()), args, Some(1))
        }
        "cot" | "cotangent" => {
            evaluate_static_math_func(|args| Ok(1f64 / args[0].tan()), args, Some(1))
        }

        // log
        "log" | "log10" => evaluate_static_math_func(|args| Ok(args[0].log10()), args, Some(1)),
        "ln" => evaluate_static_math_func(|args| Ok(args[0].ln()), args, Some(1)),

        // statistics
        "mean" => evaluate_static_math_func(
            |args| Ok(args.iter().fold(0f64, |acc, x| acc + x) / args.len() as f64),
            args,
            None,
        ),

        // note : we might actually need a better kind of error here
        _ => Err("error : was unable to evaluate builtin function".to_string()),
    }
}

/// this will invoke a math function like cosine and it will handle an error in arguments length.
fn evaluate_static_math_func(
    func: fn(&[f64]) -> Result<f64, String>,
    args: Expression,
    expected_arg_count: Option<usize>,
) -> Result<f64, String> {
    if args.len() == 0 {
        return Err("error : arguments must be supplied to a function".to_string());
    }
    let reduced_args = try_reduce_args(args)?
        .into_iter()
        .map(|token| token.to_f64().unwrap())
        .collect::<Vec<f64>>();
    if expected_arg_count.is_some_and(|arg_count| arg_count != reduced_args.len()) {
        return Err(format!(
            "expected {} args but recieved : {}",
            expected_arg_count.unwrap(),
            reduced_args.len()
        ));
    }
    func(&reduced_args)
}

#[derive(Clone, Debug)]
pub struct DynamicMathFunction {
    pub signature: Option<String>,
    pub arg_names: Vec<String>,
    pub expression: Expression,
}
impl DynamicMathFunction {
    pub fn anonymous(arg_names: Vec<String>, expression : Expression) -> Result<Self, String> {
        Ok(Self {
            signature : None,
            arg_names : arg_names,
            expression : expression
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

        eval_expression(expression_to_evaluate)
    }
}
