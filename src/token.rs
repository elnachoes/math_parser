
#[derive(Clone, Debug)]
pub enum Token {
    Number(f64),
    Operator(Operator),
    Identity(String),
}
impl Token {
    pub fn get_num(&self) -> Option<f64> {
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

    pub fn get_identity(&self) -> Option<&String> {
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

    pub fn is_operator(&self) -> bool {
        if let Self::Operator(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_argument_seperator(&self) -> bool {
        if let Self::Operator(Operator::ArgumentSeparator) = self {
            true
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
    Factorial,
    OpenParen,
    CloseParen,
    ArgumentSeparator,
    FunctionAssignment,
}
impl Operator {
    pub fn apply_operation_2_operands(&self, num1: f64, num2: f64) -> Result<f64, ()> {
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

    pub fn apply_operation_1_operand(&self, num1: f64) -> Result<f64, ()> {
        match self {
            Self::Addition => Ok(num1),
            Self::Subtraction => Ok(-num1),
            _ => Err(()),
        }
    }

    pub fn get_inverse_operator(&self) -> Option<Self> {
        match self {
            Self::Addition => Some(Self::Subtraction),
            Self::Subtraction => Some(Self::Addition),
            Self::Division => Some(Self::Multiplication),
            Self::Multiplication => Some(Self::Division),
            _ => None,
        }
    }

    pub fn is_addition_or_subtraction(&self) -> bool {
        if let Self::Addition | Self::Subtraction = self {
            true
        } else {
            false
        }
    }
}
