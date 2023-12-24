pub type Expression = Vec<Token>;

#[derive(Clone, Debug)]
pub enum Token {
    Number(f64),
    Operator(Operator),
    Identity(String),
}
impl Token {
    pub fn to_f64(&self) -> Option<f64> {
        if let Token::Number(num) = self {
            Some(*num)
        } else {
            None
        }
    }

    pub fn is_argument_separator(&self) -> bool {
        if let Self::Operator(Operator::ArgumentSeparator) = self {
            true
        } else {
            false
        }
    }

    pub fn identity_string(&self) -> Option<&str> {
        if let Self::Identity(identity) = self {
            Some(&identity)
        } else {
            None
        }
    }

    pub fn is_identity(&self) -> bool {
        if let Self::Identity(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_num(&self) -> bool {
        if let Self::Number(_) = self {
            true
        } else {
            false
        }
    }

    pub fn get_operator(&self) -> Option<&Operator> {
        if let Self::Operator(operator) = self {
            Some(operator)
        } else {
            None
        }
    }

    pub fn is_open_paren(&self) -> bool {
        if let Self::Operator(operator) = self {
            if let Operator::OpenParen = operator {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn is_close_paren(&self) -> bool {
        if let Self::Operator(operator) = self {
            if let Operator::CloseParen = operator {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulus,
    Exponentiation,
    OpenParen,
    CloseParen,
    ArgumentSeparator,
}
impl Operator {
    pub fn apply_operation(&self, num1: f64, num2: f64) -> Result<f64, ()> {
        match self {
            Self::Addition => Ok(num1 + num2),
            Self::Subtraction => Ok(num1 - num2),
            Self::Multiplication => Ok(num1 * num2),
            Self::Division => Ok(num1 / num2),
            Self::Modulus => Ok(num1 % num2),
            Self::Exponentiation => Ok(num1.powf(num2)),
            _ => Err(()),
        }
    }
}
