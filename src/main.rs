// use math_parser::Token;
// use math_parser::evaluator::*;
// use math_parser::parser::*;

use math_parser::*;

fn main() {

    println!("{:?}", parse_str("5 +     5 - 9 * (2^(3/5))"))
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

    // if let &[Token::Operator(Operator::OpenParen), Token::Number(_), Token::Operator(Operator::CloseParen)] = tokens {
    //     println!("penis")
    // } else {
    //     println!("ass")
    // }




}