use std::{default, f32::consts::E};
use core::{ops::Range, num, panic};
// use std
use crate::{
    token::*,
    parser::*
};

fn eval_str(input : &str) -> Result<f64, String> {
    let tokens = if let Ok(tokens) = parse_str(input) {
        tokens
    } else {
        return Err("error : eval_str : could not parse tokens".to_string())
    };

    Ok(0.)
}

#[derive(Clone, Debug)]
pub struct EvalState {
    pub index : usize,
    pub tokens : Vec<Token>
}
impl EvalState {
    pub fn new(tokens : Vec<Token>) -> Self {
        Self {
            index : 0,
            tokens : vec![]
        }
    }

    pub fn is_solved(&self) -> bool {
        if let &[Token::Number(_)] | &[Token::Operator(Operator::OpenParen), Token::Number(_), Token::Operator(Operator::CloseParen)] = &self.tokens.as_slice() { 
            true 
        } else { 
            false 
        }
    }

    pub fn set_index(self, new_index : usize) -> Self {
        Self { index: new_index, tokens: self.tokens }
    }

    /// this will reduce 2 number tokens and an operator token to 1 number 
    /// the number token that will replace the 3 tokens will be a result of the math operation specified by the operator
    pub fn reduce_num_op_num(self) -> Result<Self, String> {
        let mut tokens = self.tokens;
        let expression_index = self.index;
        let first_num = tokens.get(self.index);
        let operator = tokens.get(self.index + 1);
        let second_num = tokens.get(self.index + 2); 
        
        // swap the begining of the range passed in
        if first_num.is_some() && operator.is_some() && second_num.is_some() {
            let result_number_token = if let Ok(num) = operation(first_num.unwrap(), operator.unwrap(), second_num.unwrap()) {
                Token::Number(num)
            } else {
                panic!("error")
            };

            // drain 3 then insert
            tokens.drain(expression_index..=expression_index + 2);
            tokens.insert(expression_index, result_number_token);
            
            Ok(Self {
                index : self.index,
                tokens : tokens,            
            })
        } else {
            panic!("error : apply_operation_to_number_at_index : could not apply operation")
        }
    }
    
    /// this will reduce "(1)" to "1" in the eval state 
    /// the i
    pub fn reduce_paren_num_paren(self) -> Result<Self, String> {
        // NOTE : REFRACTOR OTHER METHODS TO BE LIKE THESE ONES THIS IS AWESOME
        if self.is_paren_num_paren() {
            let mut tokens = self.tokens;
            let index = self.index;
            tokens.remove(index - 1);
            tokens.remove(index);
            Ok(Self { index: index, tokens: tokens })
        } else {
            Err("error : reduce_paren_num_paren".to_string())
        }
    }

    pub fn is_paren_num_paren(&self) -> bool {
        println!("{self:?}");
        let tokens = &self.tokens;
        let index = self.index;

        let tokens_len = tokens.len();

        if tokens_len < 3 || index + 1 > tokens_len || index - 1 < 0 { return false }

        if let &[Token::Operator(Operator::OpenParen), Token::Number(_), Token::Operator(Operator::CloseParen)] = &tokens[index-1..=index+1] {
            true
        } else {
            false
        }
    }
}

/// this will evaluate all given operators for an eval_state
fn eval_operators(eval_state : EvalState, operators : &[Operator]) -> Result<EvalState, String> {
    let num_1_token_option = eval_state.tokens.get(eval_state.index);
    let operator_token_option = eval_state.tokens.get(eval_state.index + 1);
    let num_2_token_option = eval_state.tokens.get(eval_state.index + 2);

    if eval_state.is_solved() { 
        if eval_state.is_paren_num_paren() {
            return eval_state.reduce_paren_num_paren()
        } else {
            return Ok(eval_state)
        }
    }

    if num_1_token_option.is_some() && operator_token_option.is_some() {
        let operator_token = operator_token_option.unwrap();
        let operator = if let Some(operator) = operator_token.get_operator() { operator } else {
            // println!("{eval_state:?}");
            panic!("error : eval_operators")
        };

        if let Operator::CloseParen = operator {
            // this is where I need to check and see if 
            
            let new_index = eval_state.index - 1;
            let eval_state = eval_state.set_index(new_index);
            if eval_state.is_paren_num_paren() {
                return Ok(eval_state.reduce_paren_num_paren().unwrap())
            } else {
                return Ok(eval_state.set_index(new_index))
            }
        }

        if num_1_token_option.is_none() || num_2_token_option.is_none() {
            panic!("error : eval_operators")
        }

        if operators.contains(operator) {
            eval_operators(eval_state.reduce_num_op_num().unwrap(), operators)
        } else {
            eval_operators(EvalState { index: eval_state.index + 2, tokens: eval_state.tokens }, operators)
        }
    } else if num_1_token_option.is_some() && operator_token_option.is_none(){
        Ok(eval_state)
    } else {
        panic!("error : eval_operators");
    }
}




/// this will evaluate all given operators for an eval_state
fn eval_operators_test(eval_state : EvalState, operators : &[Operator]) -> Result<EvalState, String> {
    if eval_state.is_solved() { 
        if eval_state.is_paren_num_paren() {
            return eval_state.reduce_paren_num_paren()
        } else {
            return Ok(eval_state)
        }
    }

    if let &[Token::Number(num_1), Token::Operator(op)] = &eval_state.tokens[eval_state.index..=eval_state.index+1] {
        if let Operator::OpenParen = op {
            return Ok(eval_state)
        }
        
        if let Some(Token::Number(num_2)) = eval_state.tokens.get(eval_state.index+2) {

        } else {

            Ok(eval_state)
        }

    } else {

        Ok(eval_state)
    }
}



/// start at the beginning of the expression
/// step number by number looking for parens if found eval that sub expression
fn eval_sub_expression(eval_state : EvalState) -> Result<EvalState, String> {
    let token_option = eval_state.tokens.get(eval_state.index);
    if token_option.is_some() {
        if let Token::Operator(operator) = token_option.unwrap() {
            match operator {
                Operator::OpenParen => {
                    // println!("penis");
                    // recurse further and evaluate further
                    let result = eval(EvalState { 
                        index: eval_state.index + 1, 
                        tokens: eval_state.tokens 
                    });
                    
                    Ok(result.unwrap())
                },
                Operator::CloseParen => {

                    // println!("ass");
                    // reduce and return 
                    let new_index = eval_state.index - 1;
                    let eval_state = eval_state.set_index(new_index);
                    eval_state.reduce_paren_num_paren()
                },
                _ => panic!("error : eval_sub_expression")
            }
        } else {
            // println!("ass");
            let new_index = eval_state.index + 2;
            if new_index < eval_state.tokens.len() {
                eval_sub_expression(eval_state.set_index(new_index))
            } else {
                Ok(eval_state)
            }
        }
    } else {
        Ok(eval_state)
        // panic!("error : eval_sub_expression")
    }
}

// DONT WORRY ABOUT PARENS FOR NOW
pub fn eval(eval_state : EvalState) -> Result<EvalState, String> {
    // empty tokens
    if eval_state.tokens.len() == 0 { return Ok(eval_state) }
    
    let expression_start_index = eval_state.index;

    // let return_if_sub_expression_complete = |eval_state : EvalState| {  eval_state.reduce_paren_num_paren() };

    // sub expressions
    let after_sub_expression = eval_sub_expression(eval_state).unwrap();
    if after_sub_expression.is_paren_num_paren() { return after_sub_expression.reduce_paren_num_paren() }
    if after_sub_expression.is_solved() { return Ok(after_sub_expression) };

    // exp
    let after_exp = eval_operators(after_sub_expression.set_index(expression_start_index), &[Operator::Exponentiation]).unwrap();
    if after_exp.is_paren_num_paren() { return after_exp.reduce_paren_num_paren() }
    if after_exp.is_solved() { return Ok(after_exp) };

    // mult/div
    let after_mult_div = eval_operators(after_exp.set_index(expression_start_index), &[Operator::Multiplication, Operator::Division]).unwrap();
    if after_mult_div.is_paren_num_paren() { return after_mult_div.reduce_paren_num_paren() }
    if after_mult_div.is_solved() { return Ok(after_mult_div) };

    // add/sub
    let after_add_sub = eval_operators(after_mult_div.set_index(expression_start_index), &[Operator::Addition, Operator::Subtraction]).unwrap();
    if after_add_sub.is_paren_num_paren() { return after_add_sub.reduce_paren_num_paren() }
    // if after_add_sub.is_solved() { return Ok(eval_state) };
    Ok(after_add_sub)
}

fn operation(num_1 : &Token, op : &Token, num_2 : &Token) -> Result<f64, String> {
    if !num_1.is_num() || !num_2.is_num() {
        panic!("error : operation")
    }

    let f64_1 = num_1.to_f64().unwrap();
    let f64_2 = num_2.to_f64().unwrap();

    if let Token::Operator(op) = op {
        let result = match op  {
            Operator::Addition => f64_1 + f64_2,
            Operator::Subtraction => f64_1 - f64_2,
            Operator::Multiplication => f64_1 * f64_2,
            Operator::Division => f64_1 / f64_2,
            Operator::Exponentiation => f64_1.powf(f64_2),
            _ => panic!("error : operation")
        };
        Ok(result)
    } else {
        panic!("error : operation")
    }
}