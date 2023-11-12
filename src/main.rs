// use std::iter::Enumerate;
// use std::str::Chars;
// use itertools::{self, Itertools};

#[derive(Clone, Copy, Debug)]
enum Token {
    Number(f64),
    Operator(Operator),
    OpenParen,
    CloseParen,
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
    pub index : usize,
    pub input : &'a str,
    pub tokens : Vec<Token>
}

fn parse_str(input : &str) -> Result<Vec<Token>, String> {
    let cleaned_input = input.to_string().replace(" ", "");
    let parse_state = ParseState {
        index : 0,
        input : &cleaned_input,
        tokens : vec![]
    };

    match parse(parse_state) {
        Ok(tokens) => Ok(tokens),
        Err(err) => Err(err)
    }
}

fn parse(parse_state : ParseState<'_>) -> Result<Vec<Token>, String> {
    let parse_state_copy = || ParseState {
        index  : parse_state.index,
        input : parse_state.input,
        tokens : parse_state.tokens.as_slice().to_vec()
    }; 

    let parse_control = |parse_state : ParseState<'_>| {
        if parse_state.index == parse_state.input.len() {
            return Ok(parse_state.tokens)
        } else {
            return parse(parse_state)
        }
    };

    if let Ok(parse_state) = parse_char_token(parse_state_copy()) {
        return parse_control(parse_state);
    }

    if let Ok(parse_state) = parse_number_token(parse_state_copy()) {
        return parse_control(parse_state);
    }

    Err("error : parse : could not parse ".to_string())
}

fn parse_char_token(parse_state : ParseState<'_>) -> Result<ParseState, String> {
    let success = |token| Ok(ParseState {
        index : parse_state.index + 1,
        input : parse_state.input,
        tokens : [parse_state.tokens.as_slice(), &[token]].concat()
    });

    let fail = |message| Err(message);

    let char_to_parse = parse_state.input.chars().nth(parse_state.index);

    if let None =  char_to_parse {
        return fail("error : parse_operator : nothing to parse out of characters".to_string())
    }

    match char_to_parse.unwrap() {
        '+' => success(Token::Operator(Operator::Addition)),
        '-' => success(Token::Operator(Operator::Subtraction)),
        '*' => success(Token::Operator(Operator::Multiplication)),
        '/' => success(Token::Operator(Operator::Division)),
        '^' => success(Token::Operator(Operator::Exponentiation)),

        '(' => success(Token::OpenParen),
        ')' => success(Token::CloseParen),
        _ => fail("error : parse_operator : unable to parse operator from char".to_string())
    }
}

fn parse_number_token(parse_state : ParseState<'_>) -> Result<ParseState, String> {
    let mut end_number_index = parse_state.index;
    for ch in parse_state.input[parse_state.index..].chars() {
        if ch.is_numeric() || ch == '.' {
            end_number_index += 1
        } else {
            break
        }
    }

    let f64_parse_result = parse_state
        .input[parse_state.index..end_number_index]
        .parse::<f64>();
    
    if let Err(_error) = f64_parse_result {
        return Err("error : parse_number : float parse error".to_string())
    }

    Ok(ParseState { 
        index: end_number_index, 
        input: parse_state.input, 
        tokens: [parse_state.tokens.as_slice(), &[Token::Number(f64_parse_result.unwrap())]].concat()
    })
}

struct EvalState {
    pub index : usize,
    pub tokens : Vec<Token>
}

fn eval_str(input : &str) -> Result<f64, String> {
    let tokens_result = parse_str(input);

    if let Err(err) = tokens_result {
        return Err(err)
    }

    let eval_state = EvalState {
        index : 0,
        tokens : tokens_result.unwrap()
    };
    
    Err("eval".to_string())
}

// fn eval_add()

fn main() {
    println!("{:?}", parse_str("25 ^ (6*5)"));
}

