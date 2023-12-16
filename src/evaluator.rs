use std::ops::RangeInclusive;

use itertools::Itertools;

use crate::*;

enum EvalOperatorsDirection {
    LeftToRight,
    RightToLeft,
}
impl EvalOperatorsDirection {
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

    pub fn index_at_expression_end(&self, tokens: &Vec<Token>, index: usize) -> bool {
        match self {
            Self::LeftToRight => index == tokens.len() - 1,
            Self::RightToLeft => index == 0,
        }
    }

    pub fn get_starting_index(&self, tokens: &Vec<Token>) -> Result<usize, String> {
        if tokens.len() == 0 {
            return Err("error : no starting index for token string of length 0".to_string());
        }
        match self {
            Self::LeftToRight => Ok(0),
            Self::RightToLeft => Ok(tokens.len() - 1),
        }
    }
}

fn eval_operators(tokens: Vec<Token>,operators: &[Operator], eval_direction: EvalOperatorsDirection) -> Result<Vec<Token>, String> {
    let mut tokens = tokens;

    let mut index = eval_direction.get_starting_index(&tokens)?;

    loop {
        if eval_direction.index_at_expression_end(&tokens, index) || tokens.len() == 1 {
            break;
        }

        let operation_range = eval_direction.get_operation_range_at_index(index);

        let reduction_option = if let &[Token::Number(left_num), Token::Operator(op), Token::Number(right_num)] = &tokens[operation_range.clone()] {
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
            if let EvalOperatorsDirection::RightToLeft = eval_direction {
                index -= 2
            }
        } else {
            index = eval_direction.get_adjacent_operand_index(index);
        }
    }
    Ok(tokens)
}

/// this will find the end of a sub expression by itterating through the token string and finding where the scope is enclosed.
fn find_sub_expression_end(tokens: &Vec<Token>, expression_start_index: usize) -> Result<usize, String> {
    if expression_start_index >= tokens.len() || tokens.get(expression_start_index).is_none() {
        return Err("error : find_expression_end".to_string());
    }

    if expression_start_index == 0 {
        return Ok(tokens.len() - 1);
    }

    let mut current_expression_scope = 0;
    for (i, token) in tokens.iter().enumerate().skip(expression_start_index) {
        if let Token::Operator(Operator::OpenParen) = token {
            current_expression_scope += 1
        }

        if let Token::Operator(Operator::CloseParen) = token {
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
fn is_solved_token_string(tokens: &Vec<Token>) -> bool {
    if tokens.len() == 1 && tokens.first().is_some_and(|token| { if let Token::Number(_) = token { true } else { false } }) {
        true
    } else {
        false
    }
}

/// this will evaluate a token string expresssion recursively.
pub fn eval_expression(tokens: Vec<Token>) -> Result<f64, String> {
    // if the token string comes in solved return the solved answer
    if is_solved_token_string(&tokens) {
        return Ok(tokens.first().unwrap().to_f64().unwrap());
    }

    // find each sub expression and store a list of the answer and range of tokens they will replace.
    let mut sub_expression_solutions: Vec<(Token, RangeInclusive<usize>)> = vec![];
    for (index, _token) in tokens.iter().enumerate().filter(|(_index, token)| { if let Token::Operator(Operator::OpenParen) = token { true } else { false } }) {
        if sub_expression_solutions.iter().any(|(_token, range)| range.contains(&index)) {
            continue;
        }
        let pre_calc_start_index = index + 1;
        let pre_calc_end_index = find_sub_expression_end(&tokens, pre_calc_start_index)?;
        let sub_expression_result = eval_expression(tokens[pre_calc_start_index..=pre_calc_end_index].to_vec())?;
        sub_expression_solutions.push((Token::Number(sub_expression_result), index..=pre_calc_end_index + 1))
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

    let after_exp = eval_operators(after_sub_expressions, &[Operator::Exponentiation], EvalOperatorsDirection::RightToLeft)?;
    if is_solved_token_string(&after_exp) {
        return Ok(after_exp.first().unwrap().to_f64().unwrap());
    }

    let after_mult_div = eval_operators(after_exp, &[Operator::Multiplication, Operator::Division], EvalOperatorsDirection::LeftToRight)?;
    if is_solved_token_string(&after_mult_div) {
        return Ok(after_mult_div.first().unwrap().to_f64().unwrap());
    }

    let after_add_sub = eval_operators(after_mult_div, &[Operator::Addition, Operator::Subtraction], EvalOperatorsDirection::LeftToRight)?;
    if is_solved_token_string(&after_add_sub) {
        return Ok(after_add_sub.first().unwrap().to_f64().unwrap());
    }

    Err("error : unsolved expression : {}".to_string())
}

pub fn eval_str(string: &str) -> Result<f64, String> {
    let token_string = parse_str(string)?;
    eval_expression(token_string)
}