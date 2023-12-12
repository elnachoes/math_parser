use math_parser::eval_str;

fn main() -> Result<(), String> {
    println!("{}",eval_str(std::env::args().skip(1).collect::<String>().as_str())?);
    Ok(())
}