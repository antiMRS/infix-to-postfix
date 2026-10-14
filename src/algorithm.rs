use crate::{MathToken, Operator, ParseError};

macro_rules! EXPECT_VALUE {
    ($cond:expr) => {
        if !$cond {
            return Err(ParseError::UnexpectedOperand);
        }
    };
}

macro_rules! EXPECT_OPERATOR {
    ($cond:expr) => {
        if $cond {
            return Err(ParseError::UnexpectedOperand);
        }
    };
}

/// Converts an infix sequence of tokens into postfix (Reverse Polish) notation.
///
/// This function implements the shunting-yard algorithm to reorder tokens,
/// taking operator precedence, associativity, parentheses, function calls,
/// and unary operators into account.
///
/// # Arguments
///
/// * `tokens` – An iterator of `MathToken`s representing the infix expression.
///
/// # Returns
///
/// * `Ok(Vec<MathToken<V, O>>)` – The postfix token sequence.
/// * `Err(ParseError)` – If the input is invalid, e.g. unbalanced parentheses,
///   missing operands, or unexpected operands.
///
/// # Examples
///
/// ```
/// use infix-to-postfix::{into_postfix, MathToken};
/// use infix-to-postix::defaults::*;
///
/// let infix = vec![
///     MathToken::Value(DefaultValue::Integer(1)),
///     MathToken::Operator(DefaultOperator::Add),
///     MathToken::Value(DefaultValue::Integer(2)),
/// ];
/// let postfix = into_postfix(infix).unwrap();
/// assert_eq!(postfix, vec![
///     MathToken::Value(DefaultValue::Integer(1)),
///     MathToken::Value(DefaultValue::Integer(2)),
///     MathToken::Operator(DefaultOperator::Add),
/// ]);
/// ```
///
/// # Errors
///
/// See [crate::ParseError] for exclamaion.
///
pub fn into_postfix<I, V, O>(tokens: I) -> Result<Vec<MathToken<V, O>>, ParseError>
where
    I: IntoIterator<Item = MathToken<V, O>>,
    O: Operator,
{
    let mut output: Vec<MathToken<V, O>> = Vec::new();
    let mut stack: Vec<MathToken<V, O>> = Vec::new();

    let mut expect_operand = true;

    for token in tokens.into_iter() {
        match token {
            MathToken::Value(v) => {
                // The values are sent to the output immediately
                EXPECT_VALUE!(expect_operand);
                output.push(MathToken::Value(v));
                expect_operand = false;
            }

            MathToken::Call(v) => {
                // Similar to LParen
                EXPECT_VALUE!(expect_operand);
                stack.push(MathToken::Call(v));
            }

            MathToken::LParen => {
                // Moving to the top of the stack
                EXPECT_VALUE!(expect_operand);
                stack.push(MathToken::LParen);
            }

            MathToken::RParen => {
                EXPECT_OPERATOR!(expect_operand);
                // We push all the elements from the stack of operations
                // until we find the LParen
                let mut found_lparen = false;
                while let Some(top) = stack.pop() {
                    if top.is_lparen() {
                        found_lparen = true;
                        break;
                    }
                    output.push(top);
                }
                if !found_lparen {
                    return Err(ParseError::MissingParen);
                }
                // If there is a call after the left stack,
                // then we will push it into the output.
                if let Some(MathToken::Call(_)) = stack.last() {
                    output.push(stack.pop().unwrap())
                }
                expect_operand = false;
            }

            MathToken::COMMA => {
                EXPECT_OPERATOR!(expect_operand);
                // We send all operators from the stack to
                // the output up to the first LParen
                let mut found_lparen = false;
                while let Some(top) = stack.pop() {
                    if top.is_lparen() {
                        found_lparen = true;
                        break;
                    }
                    output.push(top);
                }
                if !found_lparen {
                    return Err(ParseError::MissingParen);
                }
                stack.push(MathToken::LParen);
                expect_operand = true;
            }

            MathToken::Operator(op) => {
                if op.is_unary() {
                    // unary operands have heighets precedence
                    EXPECT_VALUE!(expect_operand);
                    stack.push(MathToken::Operator(op));
                } else {
                    EXPECT_OPERATOR!(expect_operand);
                    let prec = op.precedence();
                    let right_assoc = op.is_righthand();

                    // If the now operator is not unary
                    // then we send from the stack to the output
                    // all operators whose precedence is
                    // less than or equal to the precedence of the
                    // current one.

                    while let Some(top) = stack.last() {
                        let Some(top_op) = top.as_operator() else {
                            break;
                        };
                        let top_prec = top_op.precedence();
                        let should_pop = if right_assoc {
                            top_prec > prec
                        } else {
                            top_prec >= prec
                        };
                        if should_pop {
                            output.push(stack.pop().unwrap());
                        } else {
                            break;
                        }
                    }

                    stack.push(MathToken::Operator(op));
                    expect_operand = true;
                }
            }
        }
    }

    // Make sure that all operators have received their arguments
    if expect_operand {
        return Err(ParseError::ExpectedOperand);
    }

    // Moving all remaining operators from the stack to output.
    // If one of them turns out to be LParen,
    // it means that there are non-closed brackets in the
    // original expression.
    while let Some(top) = stack.pop() {
        if top.is_paren() {
            return Err(ParseError::UnclosedParens);
        }
        output.push(top);
    }

    Ok(output)
}
