use std::default;
use core::{ops::Range, num};
// use std

#[derive(Clone, Copy, Debug)]
enum Token {
    Number(f64),
    Operator(Operator),
    OpenParen,
    CloseParen,
}
impl Token {
    pub fn to_f64(&self) -> Option<f64> {
        if let Token::Number(num) = self { Some(*num) } else { None }
    }
        
    pub fn is_num(&self) -> bool {
        if let Self::Number(_) = self { true } else { false }
    }
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
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

        '(' => Token::OpenParen,
        ')' => Token::CloseParen,
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

    pub fn reduce_range(self, range : Range<usize>, token : Token) -> Self {
        let mut tokens = self.tokens;
        tokens[range].swap_with_slice(&mut [token]);
        Self {
            index : self.index,
            tokens : tokens,            
        }
    }

}


// DONT WORRY ABOUT PARENS FOR NOW
fn eval(tokens : Vec<Token>) -> Result<f64, String> {
    // empty tokens
    if tokens.len() == 0 { return Ok(0.) }
    
    // exp

    // mult/div

    // add/sub

    
    // if let token = tokens[index]
    
    Ok(0.)
}


fn eval_exponentiation(tokens : Vec<Token>) -> Result<f64, String> {
    

    Ok(0.)
}

/// 1 + 2 - 4 + 5 
///     3 - 4 + 5   
///        -1 + 5 
///             4
fn eval_add_sub(eval_state : EvalState) -> Result<EvalState, String> {
    // eval_state.tokens

    Ok(eval_state)
}

fn operation(num_1 : &Token, op : &Token, num_2 : &Token) -> Result<f64, String> {
    if !num_1.is_num() || !num_2.is_num() {
        return Err("error : operation : there are 2 numbers in a rows".to_string());
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
        };
        Ok(result)
    } else {
        Err("error : operation : expected operator".to_string())
    }
}


fn main() {
    println!("{:?}", parse_str("33 * 4 + 66"));
}