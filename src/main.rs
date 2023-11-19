use std::{default, f32::consts::E};
use core::{ops::Range, num, panic};
// use std

#[derive(Clone, Copy, Debug)]
enum Token {
    Number(f64),
    Operator(Operator),
    // OpenParen,
    // CloseParen,
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

    pub fn is_close_paren(&self) -> bool {
        if let Self::Operator(operator) = self { if let Operator::CloseParen = operator { true } else { false } } else { false }
    }
}

#[derive(Clone, Copy, Debug)]
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

    // this will reduce 2 number tokens and an operator token to 1 number 
    // the number token that will replace the 3 tokens will be a result of the math operation specified by the operator
    pub fn apply_operation_to_number_at_index(self) -> Result<Self, String> {
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
}

// ---- todo : cleanup these functions dude ----


// 1 + 2 - 4 + 5 
//     3 - 4 + 5   
//        -1 + 5 
//             4
fn eval_add_sub(eval_state : EvalState) -> Result<EvalState, String> {
    let first_num = eval_state.tokens.get(eval_state.index);
    let operator = eval_state.tokens.get(eval_state.index + 1);
    let second_num = eval_state.tokens.get(eval_state.index + 2);
    
    // 1 
    // or
    // 1 + 2 + 3 ...
    // not
    // 1 +  

    if first_num.is_some() && operator.is_some() {
        match *operator.unwrap().get_operator().unwrap() {
            Operator::Addition | Operator::Subtraction => {
                if second_num.is_none() {
                    panic!("error :  eval_add_sub")
                }
                eval_add_sub(eval_state.apply_operation_to_number_at_index().unwrap())
            },
            Operator::CloseParen => Ok(eval_state),
            _ => eval_add_sub(EvalState { index: eval_state.index + 2, tokens: eval_state.tokens })
        }

    } else if first_num.is_some() && operator.is_none(){
        Ok(eval_state)
        // println!("{:?}", eval_state.tokens);
    } else {
        panic!("error : eval_add_sub : could not add");
    }
}

fn eval_mult_div(eval_state : EvalState) -> Result<EvalState, String> {
    let first_num = eval_state.tokens.get(eval_state.index);
    let operator = eval_state.tokens.get(eval_state.index + 1);
    let second_num = eval_state.tokens.get(eval_state.index + 2);
    
    // 1 
    // or
    // 1 + 2 + 3 ...
    // not
    // 1 +  

    if first_num.is_some() && operator.is_some() {
        match *operator.unwrap().get_operator().unwrap() {
            Operator::Multiplication | Operator::Division => {
                if second_num.is_none() {
                    panic!("error :  eval_add_sub")
                }
                eval_mult_div(eval_state.apply_operation_to_number_at_index().unwrap())
            },
            Operator::CloseParen => Ok(eval_state),
            _ => eval_mult_div(EvalState { index: eval_state.index + 2, tokens: eval_state.tokens })
        }

    } else if first_num.is_some() && operator.is_none(){
        Ok(eval_state)
    } else {
        panic!("error : eval_add_sub : could not add");
    }
}

fn eval_exp(eval_state : EvalState) -> Result<EvalState, String> {
    let first_num = eval_state.tokens.get(eval_state.index);
    let operator = eval_state.tokens.get(eval_state.index + 1);
    let second_num = eval_state.tokens.get(eval_state.index + 2);
    
    // 1 
    // or
    // 1 + 2 + 3 ...
    // not
    // 1 +  

    if first_num.is_some() && operator.is_some() {
        match *operator.unwrap().get_operator().unwrap() {
            Operator::Exponentiation => {
                if second_num.is_none() {
                    panic!("error :  eval_add_sub")
                }
                eval_exp(eval_state.apply_operation_to_number_at_index().unwrap())
            },
            Operator::CloseParen => Ok(eval_state),
            _ => eval_exp(EvalState { index: eval_state.index + 2, tokens: eval_state.tokens })
        }

    } else if first_num.is_some() && operator.is_none(){
        Ok(eval_state)
    } else {
        panic!("error : eval_add_sub : could not add");
    }
}

// 

// fn eval_mult_div(eval_state : EvalState)


// DONT WORRY ABOUT PARENS FOR NOW
// fn eval(eval_state : EvalState) -> Result<EvalState, String> {
//     // empty tokens
//     if eval_state.tokens.len() == 0 { return Ok(eval_state) }
    
//     // exp
//     let exp =

//     // mult/div

//     // add/sub

    
//     // if let token = tokens[index]
    
//     Ok(eval_state)
// }

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
    let eval_state = EvalState { index: 0, tokens: parse_str("5  * 5 + 1 - 4 / 7").unwrap() };


    let after_mult_div = eval_mult_div(eval_state);
    println!("{:?}", after_mult_div);

    let new_eval_state = EvalState {
        index : 0,
        tokens : after_mult_div.unwrap().tokens
    };

    let after_add_sub = eval_add_sub(new_eval_state);
    println!("{:?}", after_add_sub);
}