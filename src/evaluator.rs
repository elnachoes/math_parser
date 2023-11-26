use core::panic;

// use std
use crate::{parser::*, token::{*, self}};

pub fn eval_str(input: &str) -> Result<f64, String> {
    let tokens = if let Ok(tokens) = parse_str(input) {
        tokens
    } else {
        return Err("error : eval_str : could not parse tokens".to_string());
    };

    Ok(eval_expression(&tokens).unwrap())
}

fn eval_expression(tokens: &[Token]) -> Result<f64, String> {
    // step 1 : the top level expression needs to get reconstructed from ALL of the sub expressions
    // this means that each item in the passed in the passarray will get copied into a vec

    // setup the first section of evaluating an expression by copying the expression tokens into a vec for computing the expression
    let mut top_level_expression_vec = tokens.to_vec();

    // this will construct the top level expression by solving all of the sub expressions
    while let Some((i, _token)) = top_level_expression_vec.iter().enumerate().find(|(_i, token)| if let Token::Operator(Operator::OpenParen) = token {true} else {false}) {
        let sub_expression_start_index = i + 1;
        let sub_expression_end_index = find_expression_end(&top_level_expression_vec, sub_expression_start_index).unwrap();
        let sub_expression_solution = eval_expression(&top_level_expression_vec[sub_expression_start_index..=sub_expression_end_index]).unwrap();

        top_level_expression_vec.drain(sub_expression_start_index-1..=sub_expression_end_index+1);
        top_level_expression_vec.insert(sub_expression_start_index-1, Token::Number(sub_expression_solution))
    }

    // let after_exp = match eval_operators(top_level_expression_vec, 0, &[Operator::Exponentiation], EvalDirection::RightToLeft) {
    //     EvalOperatorsResult::ReducedOrUnchanged(tokens) => tokens,
    //     EvalOperatorsResult::Solved(num) => return Ok(num),
    // };

    // println!("{after_exp:?}");
    
    let after_mult_div = match eval_operators(top_level_expression_vec, 0, &[Operator::Multiplication, Operator::Division], EvalDirection::LeftToRight) {
        EvalOperatorsResult::ReducedOrUnchanged(tokens) => tokens,
        EvalOperatorsResult::Solved(num) => return Ok(num),
    };
    
    println!("{after_mult_div:?}");

    match eval_operators(after_mult_div, 0, &[Operator::Addition, Operator::Subtraction], EvalDirection::LeftToRight) {
        EvalOperatorsResult::ReducedOrUnchanged(_) => panic!("error : eval_test"),
        EvalOperatorsResult::Solved(num) => Ok(num),
    }
    // println!("asdf");
}


// TODO WE NEED A GET NEXT OPERAND FUNC
#[derive(Clone, Copy, PartialEq, Debug)]
enum EvalDirection {
    LeftToRight,
    RightToLeft
}
impl EvalDirection {
    pub fn get_next_operand_index(&self, index : usize) -> usize {
        use EvalDirection::*;
        match self {
            LeftToRight => index.saturating_add(2),
            RightToLeft => index.saturating_sub(2)
        }
    }

    pub fn is_left_to_right(&self) -> bool { if let Self::LeftToRight = self { true } else { false }}
}

enum EvalOperatorsResult {
    ReducedOrUnchanged(Vec<Token>),
    Solved(f64)
}

/// this will evaluate all given operators for an eval_state
fn eval_operators(tokens : Vec<Token>, index : usize, operators: &[Operator], eval_direction : EvalDirection) -> EvalOperatorsResult {
    if let Some(num) = try_get_expression_solution(&tokens) {
        return EvalOperatorsResult::Solved(num)
    }

    // println!("first bool : {} && {}", index >= tokens.len(), eval_direction.is_left_to_right());
    // println!("second bool : {} && {}", index <= 0, !eval_direction.is_left_to_right());
    if index >= tokens.len() && eval_direction.is_left_to_right() || index <= 0  && !eval_direction.is_left_to_right() {
        return EvalOperatorsResult::ReducedOrUnchanged(tokens)
    }

    let next_operand_index = eval_direction.get_next_operand_index(index);
    let operation_range = if next_operand_index > index { index..=next_operand_index } else { next_operand_index..=index };

    if *operation_range.end() >= tokens.len() || *operation_range.start() > 0 { return EvalOperatorsResult::ReducedOrUnchanged(tokens) }


    // let next_operand_index = if let EvalDirection::LeftToRight = eval_direction { index+2 } else { index-2 };

    // if index + 2 >= tokens.len() { return EvalOperatorsResult::ReducedOrUnchanged(tokens) }

    // println!("[{operation_range:?}]:{tokens:?}");

    if let &[Token::Number(_), Token::Operator(op), Token::Number(_)] = &tokens[operation_range.clone()] {
        if !operators.contains(&op) {
            return eval_operators(tokens, next_operand_index, operators, eval_direction)
        }
        
        if let Some(Token::Number(_)) = tokens.get(next_operand_index) {
            // println!("asdf");
            let new_reduced_tokens = reduce_operation_at_index(tokens, index, eval_direction).unwrap();
            // println!("these are the new reduced tokens {new_reduced_tokens:?}");
            return eval_operators(new_reduced_tokens, next_operand_index, operators, eval_direction)
        }
    }

    EvalOperatorsResult::ReducedOrUnchanged(tokens)
}


/// this will reduce 2 number tokens and an operator token to 1 number
/// the number token that will replace the 3 tokens will be a result of the math operation specified by the operator
fn reduce_operation_at_index(tokens : Vec<Token>, index : usize, eval_direction : EvalDirection) -> Result<Vec<Token>, String> {
    let mut tokens = tokens;

    let next_operand_index = eval_direction.get_next_operand_index(index);
    let operation_range = if next_operand_index > index { index..=next_operand_index } else { next_operand_index..=index };

    // println!("{operation_range:?}");

    // let operation_range = index..=index+2;
    if let &[Token::Number(num_1), Token::Operator(op), Token::Number(num_2)] = &tokens[operation_range.clone()] {
        if let Operator::OpenParen | Operator::CloseParen = op {
            panic!("error : reduce_num_op_num : cannot reduce operation on paren")
        }

        let operation_result = match op {
            Operator::Addition => num_1 + num_2,
            Operator::Subtraction => num_1 - num_2,
            Operator::Multiplication => num_1 * num_2,
            Operator::Division => num_1 / num_2,
            Operator::Exponentiation => num_1.powf(num_2),
            _ => panic!("error : reduce_operation_at_index")
        };

        tokens.drain(operation_range.clone());

        tokens.insert(index, Token::Number(operation_result));
        Ok(tokens)
    } else {
        panic!("error : reduce_num_op_num")
    }
}

fn is_index_at_expression_edge(tokens : &[Token], index : usize) -> bool {
    if index == tokens.len() || index == 0 { return true }
    false
}

// if an expression is only 1 number token then the expression is solved return the number
fn try_get_expression_solution(tokens : &[Token]) -> Option<f64> {
    if let &[Token::Number(num)] = tokens { Some(num) } else { None }
}

// this will find the end of an expression
fn find_expression_end(tokens : &[Token], expression_start_index : usize) -> Result<usize, String> {
    if expression_start_index >= tokens.len() || tokens.get(expression_start_index).is_none() { return Err("error : find_expression_end".to_string()) }

    if expression_start_index == 0 { return Ok(tokens.len() - 1) }

    let mut current_expression_scope = 0;
    for (i, token) in tokens.iter().enumerate().skip(expression_start_index) {
        if let Token::Operator(Operator::OpenParen) = token { current_expression_scope += 1 }

        if let Token::Operator(Operator::CloseParen) = token {
            if current_expression_scope == 0 { return Ok(i - 1) }
            if current_expression_scope != 0 { current_expression_scope -= 1 }
        }
    }

    panic!("error : find_expression_end")
}