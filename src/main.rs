mod algorithm;
mod math_token;
#[cfg(test)]
mod tests;

pub use algorithm::into_postfix;
pub use math_token::MathToken;

pub mod defaults {
    pub use crate::math_token::{DefaultMathToken, DefaultOperator, DefaultValue};
}

pub trait Operator {
    fn precedence(&self) -> u8;

    fn is_righthand(&self) -> bool;
    fn is_lefthand(&self) -> bool {
        !self.is_righthand()
    }

    fn is_binary(&self) -> bool;
    fn is_unary(&self) -> bool {
        !self.is_binary()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum ParseError {
    IncorrectMathSeq = 0,
    MissingParen = 1,
    UnclosedParens = 2,
    UnexpectedOperand = 3,
    ExpectedOperand = 4,
}

fn main() {
    use DefaultOperator::*;
    use DefaultValue::*;
    use MathToken::*;
    use defaults::*;

    println!(
        "{:?}",
        into_postfix(vec![
            Value(Integer(10)),
            Operator(Mul),
            Operator(Neg),
            Value(Integer(7))
        ])
    );
}
