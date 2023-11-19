use std::{default, f32::consts::E};
use core::{ops::Range, num, panic};
// use std

#[derive(Clone, Copy, Debug)]
enum Token {
    Number(f64),
    Operator(Operator),
}
impl Token {
    pub fn to_f64(&self) -> Option<f64> {
        if let Token::Number(num) = self { Some(*num) } else { None }
    }
        
    pub fn is_num(&self) -> bool {
        if let Self::Number(_) = self { true } else { false }
    }

    pub fn get_operator(&self) -> Option<&Operator> {
        if let Self::Operator(operator) = self { Some(operator) } else { None }
    }

    pub fn is_open_paren(&self) -> bool {
        if let Self::Operator(operator) = self { if let Operator::OpenParen = operator { true } else { false } } else { false }
    }

    pub fn is_close_paren(&self) -> bool {
        if let Self::Operator(operator) = self { if let Operator::CloseParen = operator { true } else { false } } else { false }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
    OpenParen,
    CloseParen,
}

#[derive(Clone, Debug)]
struct ParseState<'a> {
    pub index: usize,
    pub input: &'a str,
    pub tokens: Vec<Token>,
}
impl ParseState<'_> {
    pub fn new<'a>(input : &'a str) -> ParseState<'a> {
        ParseState::<'a> { 
            index : 0,
            input : input,
            tokens : vec![]
        }
    }

    pub fn push_token(self, token : Token, index_advance_amount : usize) -> Self {
        let mut tokens = self.tokens;
        tokens.push(token);
        Self {
            index : self.index + index_advance_amount,
            input : self.input,
            tokens : tokens
        }
    }
}

fn parse_str(input: &str) -> Result<Vec<Token>, String> {
    let cleaned_input = input.to_string().replace(" ", "");
    let parse_state = ParseState::new(cleaned_input.as_str());

    match parse(parse_state) {
        Ok(tokens) => Ok(tokens),
        Err(err) => Err(err),
    }
}

fn parse(parse_state: ParseState<'_>) -> Result<Vec<Token>, String> {
    let parse_control = |parse_state: ParseState<'_>| {
        if parse_state.index == parse_state.input.len() {
            Ok(parse_state.tokens)
        } else {
            parse(parse_state)
        }
    };

    match parse_char_token(parse_state) {
        Ok(parse_state) => parse_control(parse_state),
        Err((_, parse_state)) => match parse_number_token(parse_state) {
            Ok(parse_state) => parse_control(parse_state),
            Err((error, _)) => Err(error)
        }
    }
}

fn parse_char_token(parse_state : ParseState<'_>) -> Result<ParseState, (String, ParseState)> {
    let char_to_parse = parse_state.input.chars().nth(parse_state.index);

    let Some(char_to_parse) = char_to_parse else {
        return Err(("error : parse_operator : nothing to parse out of characters".to_string(), parse_state));
    };

    let token = match char_to_parse {
        '+' => Token::Operator(Operator::Addition),
        '-' => Token::Operator(Operator::Subtraction),
        '*' => Token::Operator(Operator::Multiplication),
        '/' => Token::Operator(Operator::Division),
        '^' => Token::Operator(Operator::Exponentiation),
        '(' => Token::Operator(Operator::OpenParen),
        ')' => Token::Operator(Operator::CloseParen),
        _ => return Err(("error : parse_operator : could not parse char".to_string(), parse_state))
    };

    // advance one for char size
    Ok(parse_state.push_token(token, 1))
}

fn parse_number_token(parse_state: ParseState) -> Result<ParseState, (String, ParseState)> {
    let mut end_number_index = parse_state.index;
    for ch in parse_state.input[parse_state.index..].chars() {
        if ch.is_numeric() || ch == '.' {
            end_number_index += 1
        } else {
            break;
        }
    }

    let f64_parse_result = parse_state.input[parse_state.index..end_number_index].parse::<f64>();

    if let Err(_error) = f64_parse_result {
        return Err(("error : parse_number : float parse error".to_string(), parse_state));
    }

    let index_advance_amount = end_number_index - parse_state.index;
    
    Ok(parse_state.push_token(Token::Number(f64_parse_result.unwrap()), index_advance_amount))
}

fn eval_str(input : &str) -> Result<f64, String> {
    let tokens = if let Ok(tokens) = parse_str(input) {
        tokens
    } else {
        return Err("error : eval_str : could not parse tokens".to_string())
    };

    Ok(0.)
}

#[derive(Clone, Debug)]
struct EvalState {
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

    if num_1_token_option.is_some() && operator_token_option.is_some() {
        let operator_token = operator_token_option.unwrap();
        let operator = if let Some(operator) = operator_token.get_operator() { operator } else {
            // println!("{eval_state:?}");
            panic!("error : eval_operators")
        };

        if let Operator::CloseParen = operator {
            let new_index = eval_state.index;
            return Ok(eval_state.set_index(new_index))
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

/// start at the beginning of the expression
/// step number by number looking for parens if found eval that sub expression
fn eval_sub_expression(eval_state : EvalState) -> Result<EvalState, String> {
    let token_option = eval_state.tokens.get(eval_state.index);
    if token_option.is_some() {
        if let Token::Operator(operator) = token_option.unwrap() {
            match operator {
                Operator::OpenParen => {
                    // println!("{eval_state:?}");
                    // recurse further and evaluate further
                    let result = eval(EvalState { 
                        index: eval_state.index + 1, 
                        tokens: eval_state.tokens 
                    });
                    
                    Ok(result.unwrap())
                },
                Operator::CloseParen => {
                    // reduce and return 
                    let new_index = eval_state.index - 1;
                    let eval_state = eval_state.set_index(new_index);
                    eval_state.reduce_paren_num_paren()
                },
                _ => panic!("error : eval_sub_expression")
            }
        } else {
            let new_index = eval_state.index + 2;
            eval_sub_expression(eval_state.set_index(new_index))
        }
    } else {
        Ok(eval_state)
        // panic!("error : eval_sub_expression")
    }
}

// DONT WORRY ABOUT PARENS FOR NOW
fn eval(eval_state : EvalState) -> Result<EvalState, String> {
    // empty tokens
    if eval_state.tokens.len() == 0 { return Ok(eval_state) }
    
    let expression_start_index = eval_state.index;

    // let return_if_sub_expression_complete = |eval_state : EvalState| {  eval_state.reduce_paren_num_paren() };

    // sub expressions
    let after_sub_expression = eval_sub_expression(eval_state).unwrap();
    if after_sub_expression.is_paren_num_paren() { return after_sub_expression.reduce_paren_num_paren() }

    // exp
    let after_exp = eval_operators(after_sub_expression.set_index(expression_start_index), &[Operator::Exponentiation]).unwrap();
    if after_exp.is_paren_num_paren() { return after_exp.reduce_paren_num_paren() }

    // mult/div
    let after_mult_div = eval_operators(after_exp.set_index(expression_start_index), &[Operator::Multiplication, Operator::Division]).unwrap();
    if after_mult_div.is_paren_num_paren() { return after_mult_div.reduce_paren_num_paren() }

    // add/sub
    let after_add_sub = eval_operators(after_mult_div.set_index(expression_start_index), &[Operator::Addition, Operator::Subtraction]).unwrap();
    if after_add_sub.is_paren_num_paren() { return after_add_sub.reduce_paren_num_paren() }

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


fn main() {

    // println!("{:?}", parse_str("5 +     5 - 9 * (2^(3/5))"))
    let eval_state = EvalState { index: 0, tokens: parse_str("(5 * 5)").unwrap() };
    // let eval_state = EvalState { index: 0, tokens: parse_str("3^3").unwrap() };
    // let eval_state = EvalState { index: 1, tokens: parse_str("(4000)").unwrap() };

    // let after_mult_div = eval_mult_div(eval_state);
    // println!("{:?}", after_mult_div);

    // let new_eval_state = EvalState {
    //     index : 0,
    //     tokens : after_mult_div.unwrap().tokens
    // };

    // let after_add_sub = eval_add_sub(new_eval_state);
    // println!("{:?}", after_add_sub);

    // let after_exp = eval_operators(eval_state, &[Operator::Exponentiation]);
    // println!("{:?}", after_exp);

    // let after_mult_div = eval_operators(after_exp.unwrap().set_index(0), &[Operator::Multiplication, Operator::Division]);
    // println!("{:?}", after_mult_div);

    // let after_add_sub = eval_operators(after_mult_div.unwrap().set_index(0), &[Operator::Addition, Operator::Subtraction]);
    // println!("{:?}", after_add_sub);

    let eval_state = eval(eval_state).unwrap();
    println!("{eval_state:?}")

    // println!("{:?}", eval_state.reduce_paren_num_paren())
}