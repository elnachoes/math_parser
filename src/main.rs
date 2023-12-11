use math_parser::{evaluator::*, parse_str};

fn main() {
    let raw_token_string = parse_str(
        // "9+9 * (3 * 4 * ( 5 ^ 2))"
        "9+9 * (3 * 4 * ( 5 ^ 2 ^ 5    ))"
        // "(9+9) * 9"
        // "(9+9) * (9)"

    );
    // println!("raw_token_string : {raw_token_string:?}");
    println!("{:?}", eval_expression(raw_token_string.unwrap()));
}
