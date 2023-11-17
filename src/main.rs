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
    pub index: usize,
    pub input: &'a str,
    pub tokens: Vec<Token>,
}
impl ParseState<'_> {
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
    let parse_state = ParseState {
        index: 0,
        input: &cleaned_input,
        tokens: vec![],
    };

    match parse(parse_state) {
        Ok(tokens) => Ok(tokens),
        Err(err) => Err(err),
    }
}

fn parse(parse_state: ParseState<'_>) -> Result<Vec<Token>, String> {
    let parse_control = |parse_state: ParseState<'_>| {
        if parse_state.index == parse_state.input.len() {
            return Ok(parse_state.tokens);
        } else {
            return parse(parse_state);
        }
    };

    match parse_char_token(parse_state) {
        Ok(parse_state) => parse_control(parse_state),
        Err((_, parse_state)) => match parse_number_token(parse_state) {
            Ok(parse_state) => parse_control(parse_state),
            Err((error, _)) => return Err(error)
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
        _ => return Err(("error : parse_operator : nothing to parse out of characters".to_string(), parse_state))
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

fn eval(tokens : &Vec<Token>, index : usize) -> f64 {
    if tokens.len() == 0 { return 0.0 }

    // if let token = tokens[index]
    
    0.
}

// fn eval_sub_expressions(tokens : &Vec<Token>, index : usize) -> f64 {

//     0.0
// }

// fn 



fn main() {

    // let parse_state = ParseState {
    //     index : 1,
    //     input : "6",
    //     tokens : vec![Token::Number(6.)]
    // };

    // match parse_char_token(parse_state) {
    //     Ok(parse_state) => println!("{parse_state:?}"),
    //     Err(error) => println!("{error:?}")
    // }

    println!("{:?}", parse_str("33 * 4"));
}
