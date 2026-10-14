//! # Infix to Postfix Converter
//!
//! This crate provides a function to convert mathematical expressions from
//! infix notation (e.g., `1 + 2 * 3`) to postfix or Reverse Polish notation
//! (e.g., `1 2 3 * +`) using the shunting-yard algorithm.
//!
//! The original algorithm was described by Edsger W. Dijkstra in 1961.
//! You can find the original paper here:
//! [https://www.cs.utexas.edu/~EWD/MCReps/MR35.PDF](https://www.cs.utexas.edu/~EWD/MCReps/MR35.PDF)
//!
//! The crate is generic over the value type `V` and the operator type `O`,
//! which must implement the [`Operator`] trait. A set of default types
//! ([`DefaultValue`], [`DefaultOperator`], and [`DefaultMathToken`]) is
//! provided for convenience.
//!
//! # Examples
//!
//! ## Basic arithmetic
//!
//! Convert a simple addition:
//!
//! ```
//! use infix-to-postfix::defaults::*;
//! use infix-to-postfix::{into_postfix, MathToken};
//!
//! let infix = vec![
//!     MathToken::Value(DefaultValue::Integer(1)),
//!     MathToken::Operator(DefaultOperator::Add),
//!     MathToken::Value(DefaultValue::Integer(2)),
//! ];
//! let postfix = into_postfix(infix).unwrap();
//! assert_eq!(postfix, vec![
//!     MathToken::Value(DefaultValue::Integer(1)),
//!     MathToken::Value(DefaultValue::Integer(2)),
//!     MathToken::Operator(DefaultOperator::Add),
//! ]);
//! ```
//!
//! ## Operator precedence
//!
//! Multiplication has higher precedence than addition:
//!
//! ```
//! use infix-to-postfix::defaults::*;
//! use infix-to-postfix::{into_postfix, MathToken};
//!
//! let infix = vec![
//!     MathToken::Value(DefaultValue::Integer(1)),
//!     MathToken::Operator(DefaultOperator::Add),
//!     MathToken::Value(DefaultValue::Integer(2)),
//!     MathToken::Operator(DefaultOperator::Mul),
//!     MathToken::Value(DefaultValue::Integer(3)),
//! ];
//! let postfix = into_postfix(infix).unwrap();
//! assert_eq!(postfix, vec![
//!     MathToken::Value(DefaultValue::Integer(1)),
//!     MathToken::Value(DefaultValue::Integer(2)),
//!     MathToken::Value(DefaultValue::Integer(3)),
//!     MathToken::Operator(DefaultOperator::Mul),
//!     MathToken::Operator(DefaultOperator::Add),
//! ]);
//! ```
//!
//! ## Parentheses and unary operators
//!
//! Parentheses override precedence, and unary operators are supported:
//!
//! ```
//! use infix-to-postfix::defaults::*;
//! use infix-to-postfix::{into_postfix, MathToken};
//!
//! let infix = vec![
//!     MathToken::Operator(DefaultOperator::Neg),
//!     MathToken::LParen,
//!     MathToken::Value(DefaultValue::Integer(1)),
//!     MathToken::Operator(DefaultOperator::Add),
//!     MathToken::Value(DefaultValue::Integer(2)),
//!     MathToken::RParen,
//! ];
//! let postfix = into_postfix(infix).unwrap();
//! assert_eq!(postfix, vec![
//!     MathToken::Value(DefaultValue::Integer(1)),
//!     MathToken::Value(DefaultValue::Integer(2)),
//!     MathToken::Operator(DefaultOperator::Add),
//!     MathToken::Operator(DefaultOperator::Neg),
//! ]);
//! ```
//!
//! # Errors
//!
//! The conversion returns a [`ParseError`] if the input is not a valid
//! expression, for example due to unbalanced parentheses or missing operands.

mod algorithm;
mod math_token;
#[cfg(test)]
mod tests;

pub use algorithm::into_postfix;
pub use math_token::MathToken;

/// A module that provides default implementations of common types used for
/// parsing mathematical expressions.
pub mod defaults {
    pub use crate::math_token::{DefaultMathToken, DefaultOperator, DefaultValue};
}

/// A trait for operators used in mathematical or logical expressions.
///
/// Operators are characterized by three main properties:
///
/// * **Precedence** – It is used to describe the order of actions.
/// * **Associativity** – whether the operator groups operands from the left or from the right
/// * **Arity** – whether the operator is unary (takes one operand) or binary (takes two operands)
///
/// ## Examples of operators
///
/// Addition ( + ) is a righthand binary operand
///
/// Right shift ( << ) is a lefthand binary operand
///
/// In mathematics, the (-) operator can be either unary or binary,
/// so it must be set using two different operators.
///
/// Use [defaults::DefaultOperator] as an example of a correct trait implementation.
///
pub trait Operator {
    /// Returns the precedence of the operator.
    ///
    /// Higher values indicate that the operator binds tighter
    ///
    /// ## Example
    ///
    /// '*' has higher precedence then '+'
    fn precedence(&self) -> u8;

    /// Returns `true` if the operator is right-associative
    fn is_righthand(&self) -> bool;
    /// Returns `true` if the operator is left-associative
    fn is_lefthand(&self) -> bool {
        !self.is_righthand()
    }

    /// Returns `true` if the operator is binary (takes two operands)
    fn is_binary(&self) -> bool;
    /// Returns `true` if the operator is unary (takes one operand)
    fn is_unary(&self) -> bool {
        !self.is_binary()
    }
}

/// Errors that can occur while parsing a sequence of mathematical tokens into
/// postfix (Reverse Polish) notation.
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum ParseError {
    /// The token sequence does not form a valid mathematical expression.
    ///
    /// This error is a general fallback for invalid sequences that do not fit
    /// the other more specific error categories
    IncorrectMathSeq = 0,

    /// A closing parenthesis `)` or a comma `,` was encountered without a
    /// corresponding opening parenthesis `(`.
    ///
    /// ## Example
    ///
    /// ```text
    /// 1 + 2)   // Missing opening parenthesis
    /// ```
    MissingParen = 1,

    /// One or more opening parentheses `(` were not closed by the end of the
    /// token sequence.
    ///
    /// ## Example
    ///
    /// ```text
    /// (1 + 2   // Unclosed parenthesis
    /// ```
    UnclosedParens = 2,

    /// An operand (value) was encountered when the parser expected an operator
    ///
    /// ## Example
    ///
    /// ```text
    /// 1 + 2 2
    /// ```
    UnexpectedOperand = 3,

    /// The parser expected an operand (value) but encountered a different
    /// token, such a operator or right parenthesis.
    ///
    /// ## Example
    ///
    /// ```text
    /// 1 + ( 1 + )
    /// ```
    /// or
    /// ```text
    /// 1 -
    /// ```
    ExpectedOperand = 4,
}
