use math_parser::*;

fn main() -> Result<(), String> {
    println!(
        "{:?}",
        eval_str(std::env::args().skip(1).collect::<String>().as_str())?
    );

    // testing dont commit me please.
    // if you need to commit something make a test.

    // println!("{:?}", eval_str("cos(-4 * 5)"));

    // let penis_function = DefinedFunction::parse_str("penis(n):n+1")?;

    // println!("{:?}", penis_function.evaluate(&[2f64]));

    // println!("{:?}", math_definitions());

    Ok(())
}
