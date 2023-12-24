use crate::token::*;

#[derive(Clone, Debug)]
struct ParseState<'a> {
    pub index: usize,
    pub input: &'a str,
    pub tokens: Vec<Token>,
}
impl ParseState<'_> {
    pub fn new<'a>(input: &'a str) -> ParseState<'a> {
        ParseState::<'a> {
            index: 0,
            input: input,
            tokens: vec![],
        }
    }

    pub fn push_token(self, token: Token, index_advance_amount: usize) -> Self {
        let mut tokens = self.tokens;
        tokens.push(token);
        Self {
            index: self.index + index_advance_amount,
            input: self.input,
            tokens: tokens,
        }
    }
}

pub fn parse_str(input: &str) -> Result<Vec<Token>, String> {
    let cleaned_input = input.to_string().replace(" ", "");
    let parse_state = ParseState::new(cleaned_input.as_str());
    parse(parse_state)
}

fn parse(parse_state: ParseState<'_>) -> Result<Vec<Token>, String> {
    let parse_control = |parse_state: ParseState<'_>| {
        if parse_state.index >= parse_state.input.len() {
            Ok(parse_state.tokens)
        } else {
            parse(parse_state)
        }
    };

    let parse_state = match try_parse_operator_token(parse_state) {
        Ok(parse_state) => return parse_control(parse_state),
        Err((_, parse_state)) => parse_state,
    };

    let parse_state = match parse_number_token(parse_state) {
        Ok(parse_state) => return parse_control(parse_state),
        Err((_, parse_state)) => parse_state,
    };

    match parse_identity_token(parse_state) {
        Ok(parse_state) => parse_control(parse_state),
        Err((error, _)) => Err(error),
    }
}

fn try_parse_operator_token(
    parse_state: ParseState<'_>,
) -> Result<ParseState, (String, ParseState)> {
    let char_to_parse = parse_state.input.chars().nth(parse_state.index);

    if char_to_parse.is_none() {
        return Err((
            "error : parse_operator : nothing to parse out of characters".to_string(),
            parse_state,
        ));
    }

    let token = match char_to_parse.unwrap() {
        '+' => Token::Operator(Operator::Addition),
        '-' => Token::Operator(Operator::Subtraction),
        '*' => Token::Operator(Operator::Multiplication),
        '/' => Token::Operator(Operator::Division),
        '^' => Token::Operator(Operator::Exponentiation),
        '%' => Token::Operator(Operator::Modulus),
        '(' => Token::Operator(Operator::OpenParen),
        ')' => Token::Operator(Operator::CloseParen),
        ',' => Token::Operator(Operator::ArgumentSeparator),
        _ => {
            return Err((
                "error : parse_operator : could not parse char".to_string(),
                parse_state,
            ))
        }
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

    if f64_parse_result.is_err() {
        return Err((
            "error : parse_number : float parse error".to_string(),
            parse_state,
        ));
    }

    let index_advance_amount = end_number_index - parse_state.index;

    Ok(parse_state.push_token(
        Token::Number(f64_parse_result.unwrap()),
        index_advance_amount,
    ))
}

fn parse_identity_token(parse_state: ParseState) -> Result<ParseState, (String, ParseState)> {
    if !parse_state
        .input
        .chars()
        .nth(parse_state.index)
        .is_some_and(|ch| ch.is_alphabetic())
    {
        return Err((
            "error : parse_identity_token : coulnd't parse identity".to_string(),
            parse_state,
        ));
    }

    let mut end_identity_index = parse_state.index;
    for ch in parse_state.input[parse_state.index..].chars() {
        if ch.is_alphabetic() {
            end_identity_index += 1
        } else {
            break;
        }
    }

    let identity_string = parse_state.input[parse_state.index..end_identity_index].to_string();

    let index_advance_amount = end_identity_index - parse_state.index;

    Ok(parse_state.push_token(Token::Identity(identity_string), index_advance_amount))
}
