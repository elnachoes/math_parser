use math_parser::*;

fn main() -> Result<(), String> {
    // println!("{:?}",parse_str(std::env::args().skip(1).collect::<String>().as_str())?);
    println!("{:?}",eval_str(std::env::args().skip(1).collect::<String>().as_str())?);

    Ok(())
}
