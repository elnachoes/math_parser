use crate::*;

/// this maps built in functions to signatures for trig/alc/calc/stats/etc functions
pub fn try_eval_builtin_math_function(signature: &str, args: Expression) -> Result<f64, String> {
    match signature {
        // alg
        "sqrt" => evaluate_static_math_func(|args| Ok(args[0].sqrt()), args, Some(1)),
        "cbrt" => evaluate_static_math_func(|args| Ok(args[0].cbrt()), args, Some(1)),
        "pow" => evaluate_static_math_func(|args| Ok(args[0].powf(args[1])), args, Some(2)),
        "abs" => evaluate_static_math_func(|args| Ok(args[0].abs()), args, Some(1)),

        // trig
        "sin" | "sine" => evaluate_static_math_func(|args| Ok(args[0].sin()), args, Some(1)),
        "cos" | "cosine" => evaluate_static_math_func(|args| Ok(args[0].cos()), args, Some(1)),
        "tan" | "tangent" => evaluate_static_math_func(|args| Ok(args[0].tan()), args, Some(1)),
        "sec" | "secant" => {
            evaluate_static_math_func(|args| Ok(1f64 / args[0].cos()), args, Some(1))
        }
        "csc" | "cosecant" => {
            evaluate_static_math_func(|args| Ok(1f64 / args[0].sin()), args, Some(1))
        }
        "cot" | "cotangent" => {
            evaluate_static_math_func(|args| Ok(1f64 / args[0].tan()), args, Some(1))
        }
        "sinh" => evaluate_static_math_func(|args| Ok(args[0].sinh()), args, Some(1)),
        "cosh" => evaluate_static_math_func(|args| Ok(args[0].cosh()), args, Some(1)),
        "tanh" => evaluate_static_math_func(|args| Ok(args[0].tanh()), args, Some(1)),

        // log
        "log" | "log10" => evaluate_static_math_func(|args| Ok(args[0].log10()), args, Some(1)),
        "log2" => evaluate_static_math_func(|args| Ok(args[0].log2()), args, Some(1)),
        "ln" => evaluate_static_math_func(|args| Ok(args[0].ln()), args, Some(1)),

        // statistics
        "mean" => evaluate_static_math_func(
            |args| Ok(args.iter().fold(0f64, |acc, x| acc + x) / args.len() as f64),
            args,
            None,
        ),

        // note : we might actually need a better kind of error here
        _ => Err("error : was unable to evaluate builtin function".to_string()),
    }
}

/// this will invoke a math function like cosine and it will handle an error in arguments length.
fn evaluate_static_math_func(
    func: fn(&[f64]) -> Result<f64, String>,
    args: Expression,
    expected_arg_count: Option<usize>,
) -> Result<f64, String> {
    if args.len() == 0 {
        return Err("error : arguments must be supplied to a function".to_string());
    }
    let reduced_args = try_reduce_args(args)?
        .into_iter()
        .map(|token| token.get_num().unwrap())
        .collect::<Vec<f64>>();
    if expected_arg_count.is_some_and(|arg_count| arg_count != reduced_args.len()) {
        return Err(format!(
            "expected {} args but recieved : {}",
            expected_arg_count.unwrap(),
            reduced_args.len()
        ));
    }
    func(&reduced_args)
}
