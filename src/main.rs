// use math_parser::Token;
use math_parser::evaluator::*;
use math_parser::parser::*;

use math_parser::*;

fn main() {
    // println!("{:?}", parse_str("5 +     5 - 9 * (2^(3/5))"))
    // let eval_state = EvalState { index: 0, tokens: parse_str("(5 * (5 * 5))").unwrap() };
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

    // let eval_state = eval(eval_state).unwrap();
    // println!("{eval_state:?}")

    // println!("{:?}", eval_state.reduce_paren_num_paren())

    // let tokens = &[
    //     Token::Operator(Operator::OpenParen),
    //     Token::Number(25.),
    //     Token::Operator(Operator::OpenParen),
    // ];

    // let eval_state = EvalState {
    //     index: 0,
    //     tokens: vec![
    //         // Token::Operator(Operator::OpenParen),
    //         Token::Number(5.),
    //         Token::Operator(Operator::Addition),
    //         Token::Number(5.),
    //         Token::Operator(Operator::Multiplication),
    //         Token::Number(5.),
    //         Token::Operator(Operator::Addition),
    //         Token::Number(69.),
    //         // Token::Operator(Operator::CloseParen),
    //         // Token::Operator(Operator::Addition),
    //         // Token::Number(5.)
    //     ],
    // };



    // println!(
    //     "{:?}",
    //     eval_operators_test(eval_state, &[
    //         Operator::Addition,
    //         Operator::Subtraction
    //     ])
    // )

    // let tokens = parse_str("1 + (2 * 3 * (4 * 4)) + 5").unwrap().as_slice();
    // let test_eval_state = EvalState {
    //     expression_start_index : 0,
    //     expression_end_index : tokens.len(),
    //     index : 0,
    //     tokens : tokens
    // };

    // let x = find_expression_end(&test_eval_state.tokens, 0);

    // println!("{:?}", &test_eval_state.tokens[9..=12].to_vec());
    // println!("{:?}", x);

    println!("{:?}", eval_str("2^3^4"))
}
