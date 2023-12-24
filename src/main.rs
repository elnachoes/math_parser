use math_parser::*;

fn main() -> Result<(), String> {
    // ---- testing eval and testing parser ----
    // println!("{:?}",parse_str(std::env::args().skip(1).collect::<String>().as_str())?);
    // println!("{:?}",eval_str(std::env::args().skip(1).collect::<String>().as_str())?);

    // ---- testing user defined functions ----

    // let mut user_defined_function = DynamicMathFunction {
    //     signature: Some("asdf".to_string()),
    //     arg_names: vec!["arg".to_string()],
    //     expression_string: "arg + 2 * 20".to_string(),
    //     expression: None,
    // };

    // println!(
    //     "{:?}",
    //     user_defined_function.evaluate(vec![Token::Number(5f64)])
    // );

    Ok(())
}
