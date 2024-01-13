use crate::*;
use itertools::Itertools;
use std::{ops::RangeInclusive, collections::HashMap};

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
    expression: Expression,
    operators: &[Operator],
    eval_direction: NumOpNumDirection,
) -> Result<Expression, String> {
    let mut expression = expression;

    let mut index = eval_direction.get_starting_index(&expression)?;

    loop {
        if eval_direction.index_at_expression_end(&expression, index) || expression.len() == 1 {
            break;
        }

        let operation_range = eval_direction.get_operation_range_at_index(index);

        let reduction_option = if let &[Token::Number(left_num), Token::Operator(op), Token::Number(right_num)] =
            &expression[operation_range.clone()]
        {
            if operators.iter().contains(&op) {
                op.apply_operation_2_operands(left_num, right_num)
            } else {
                Err(())
            }
        } else {
            Err(())
        };

        if let Ok(result) = reduction_option {
            let insertion_index = *operation_range.start();
            expression.drain(operation_range.clone());
            let new_number_token = Token::Number(result);
            if expression.len() == 0 {
                expression.push(new_number_token)
            } else {
                expression.insert(insertion_index, new_number_token)
            }
            if let NumOpNumDirection::RightToLeft = eval_direction {
                index -= 2
            }
        } else {
            index = eval_direction.get_adjacent_operand_index(index);
        }
    }
    Ok(expression)
}

// 1
// todo : build a reduce addition subtraction operators
pub fn reduce_addition_subtraction_operators(expression: Expression) -> Expression {
    let mut expression = expression;

    let mut ranges_to_replace: Vec<(RangeInclusive<usize>, Token)> = Vec::new();
    for (index, token) in expression.iter().enumerate().filter(|(_index, token)| {
        if let Token::Operator(Operator::Addition | Operator::Subtraction) = token {
            true
        } else {
            false
        }
    }) {
        if ranges_to_replace.last().is_none()
            | ranges_to_replace
                .last()
                .is_some_and(|(range, _token)| range.end() + 1 < index)
            && token
                .get_operator()
                .is_some_and(|operator| operator.is_addition_or_subtraction())
        {
            ranges_to_replace.push((index..=index, token.clone()));
            continue;
        }

        if ranges_to_replace
            .last()
            .is_some_and(|(range, _token)| range.end() + 1 == index)
        {
            if let Token::Operator(Operator::Subtraction) = token {
                let new_operator = ranges_to_replace
                    .last()
                    .unwrap()
                    .1
                    .get_operator()
                    .unwrap()
                    .get_inverse_operator()
                    .unwrap();
                ranges_to_replace.last_mut().unwrap().1 = Token::Operator(new_operator)
            }
            ranges_to_replace.last_mut().unwrap().0 = *ranges_to_replace.last().unwrap().0.start()
                ..=*ranges_to_replace.last().unwrap().0.end() + 1
        }
    }

    let mut range_offset = 0;
    for (range, token) in ranges_to_replace {
        let index = *range.start() - range_offset;
        expression.drain(range.start() - range_offset..=range.end() - range_offset);
        expression.insert(index, token);
        range_offset += range.count() - 1;
    }

    expression
}

fn reduce_first_addition_subtraction_operators(expression: Expression) -> Expression {
    if expression.len() == 1 {
        return expression;
    }

    let mut expression = expression;
    match expression[0..2] {
        [Token::Operator(Operator::Addition), Token::Number(_)] => {
            expression.remove(0);
        }
        [Token::Operator(Operator::Subtraction), Token::Number(num)] => {
            expression.drain(0..2);
            expression.insert(0, Token::Number(-num))
        }
        _ => {}
    }
    expression
}

/// this will find the end of a sub expression by itterating through the token string and finding where the scope is enclosed.
fn find_sub_expression_end(
    expression: &Expression,
    expression_start_index: usize,
) -> Result<usize, String> {
    if expression_start_index >= expression.len()
        || expression.get(expression_start_index).is_none()
    {
        return Err("error : find_expression_end".to_string());
    }

    if expression_start_index == 0 {
        return Ok(expression.len() - 1);
    }

    let mut current_expression_scope = 0;
    for (i, token) in expression.iter().enumerate().skip(expression_start_index) {
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
fn is_solved_token_string(expression: &Expression) -> bool {
    if expression.len() == 1 && expression.first().is_some_and(|token| token.is_num()) {
        true
    } else {
        false
    }
}

/// this will evaluate a string math expression
pub fn eval_str(string: &str) -> Result<f64, String> {
    let token_string = parse_str(string)?;
    eval_expression(token_string, &MathDefinition::default_math_definitions(), true)
}


fn flatten_constants<'a>(expression: Expression, math_definitions : &HashMap<&'a str, MathDefinition>) -> Expression {
    expression
        .into_iter()
        .map(|token| 
            match token {
                Token::Identity(identity) if math_definitions
                    .get_key_value(identity.as_str())
                    .is_some_and(|(_, math_definition)| math_definition.is_constant()) => Token::Number(math_definitions.get(identity.as_str()).unwrap().get_constant().unwrap()),
                _ => token
            }
        )
        .collect()
}

/// this will evaluate a token string expresssion recursively.
pub fn eval_expression<'a>(mut expression: Expression, math_definitions : &HashMap<&'a str, MathDefinition>, first_call: bool) -> Result<f64, String> {
    if first_call {
        // TODO : flatten constants here 
        expression = reduce_addition_subtraction_operators(expression);
        expression = flatten_constants(expression, math_definitions)
    }

    // get rid of the first add/sub operator in something like : "- 1 + 2"
    expression = reduce_first_addition_subtraction_operators(expression);

    // if the token string comes in solved return the solved answer
    if is_solved_token_string(&expression) {
        return Ok(expression.first().unwrap().get_num().unwrap());
    }

    // find each sub expression and store a list of the answer and range of tokens they will replace.
    let mut sub_expression_solutions: Vec<(Token, RangeInclusive<usize>)> = vec![];
    for (index, _token) in expression
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
        let pre_calc_end_index = find_sub_expression_end(&expression, pre_calc_start_index)?;

        // if the prior token to the open paren is an identity, a function is being invoked and must be solved, otherwise solve a sub expression.
        let sub_expression_result_range = if index != 0
            && expression
                .iter()
                .nth(index - 1)
                .is_some_and(|token| token.is_identity())
        {
            let function_signature = expression
                .iter()
                .nth(index - 1)
                .unwrap()
                .get_identity()
                .unwrap();

            // let function = math_definitions.get(function_signature)


            // TODO : fix this to use the new functions registry
            let math_definition = math_definitions
                .get(function_signature.as_str())
                .ok_or(format!("could not find math definition with signature {function_signature:?}"))?;

            let args = try_reduce_args(expression[pre_calc_start_index..=pre_calc_end_index].to_vec(), &math_definitions)?
                .into_iter()
                .map(|token| token.get_num().unwrap())
                .collect::<Vec<f64>>();

            let function_result = match math_definition {
                MathDefinition::BuiltInFunction(function) => function.evaluate(&args, math_definitions)?,
                MathDefinition::DefinedFunction(function) => function.evaluate(&args, math_definitions)?,
                _ => return Err("asdf".to_string())
            };
            (Token::Number(function_result), index - 1..=pre_calc_end_index + 1)
        } else {
            let sub_expression_result = eval_expression(
                expression[pre_calc_start_index..=pre_calc_end_index].to_vec(),
                math_definitions,
                false,
            )?;
            (
                Token::Number(sub_expression_result),
                index..=pre_calc_end_index + 1,
            )
        };

        sub_expression_solutions.push(sub_expression_result_range)
    }

    // replace each token in the string with
    let mut after_sub_expressions = expression;
    let mut reduction_offset = 0;
    for (token, range) in sub_expression_solutions {
        let offset_range = range.start() - reduction_offset..=range.end() - reduction_offset;
        after_sub_expressions.drain(offset_range.clone());
        reduction_offset += offset_range.clone().count() - 1;
        after_sub_expressions.insert(*offset_range.start(), token);
    }
    if is_solved_token_string(&after_sub_expressions) {
        return Ok(after_sub_expressions.first().unwrap().get_num().unwrap());
    }

    let after_exp = eval_num_op_num_operators(
        after_sub_expressions,
        &[Operator::Exponentiation],
        NumOpNumDirection::RightToLeft,
    )?;
    if is_solved_token_string(&after_exp) {
        return Ok(after_exp.first().unwrap().get_num().unwrap());
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
        return Ok(after_mult_div.first().unwrap().get_num().unwrap());
    }

    let after_add_sub = eval_num_op_num_operators(
        after_mult_div,
        &[Operator::Addition, Operator::Subtraction],
        NumOpNumDirection::LeftToRight,
    )?;
    if is_solved_token_string(&after_add_sub) {
        return Ok(after_add_sub.first().unwrap().get_num().unwrap());
    }

    Err("error : unsolved expression : {}".to_string())
}

pub fn try_reduce_args(expression: Expression, math_definitions : &HashMap<&str, MathDefinition>) -> Result<Expression, String> {
    let reduced_arguments: Vec<Result<f64, String>> = expression
        .split(|token| token.is_argument_separator())
        .map(|expression| {
            eval_expression(expression.to_vec(), math_definitions, false)
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
