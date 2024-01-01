#[cfg(test)]
mod evaluator_tests {
    use crate::eval_str;

    #[test]
    fn eval_with_sub_expressions_and_functions() {
        assert_eq!(
            40f64 * (25f64.log10() / 2f64),
            eval_str("40 * ( log(25) / 2)").unwrap()
        );
    }

    #[test]
    fn eval_mean_builtin_function() {
        assert_eq!(
            (1f64 + 2f64 + 5f64.cos() + (4f64 - 20f64) + (5f64 + 11f64).cbrt()) / 5f64,
            eval_str("mean(1,2,cos(5),4 - 20, cbrt(5+11)   )").unwrap()
        )
    }

    #[test]
    fn eval_reduced_addition_subtraction_expression() {
        assert_eq!(1f64+1f64, eval_str("1-+-+-+-1").unwrap())
    }
}
