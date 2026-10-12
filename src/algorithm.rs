use crate::{MathToken, Operator, ParseError};

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
                if !expect_operand {
                    return Err(ParseError::UnexpectedOperand);
                }
                output.push(MathToken::Value(v));
                expect_operand = false;
            }

            MathToken::Call(v) => {
                if !expect_operand {
                    return Err(ParseError::UnexpectedOperand);
                }
                stack.push(MathToken::Call(v));
            }

            MathToken::LParen => {
                if !expect_operand {
                    return Err(ParseError::UnexpectedOperand);
                }
                stack.push(MathToken::LParen);
            }

            MathToken::RParen => {
                if expect_operand {
                    return Err(ParseError::ExpectedOperand);
                }
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
                // push call to stack
                if let Some(MathToken::Call(_)) = stack.last() {
                    output.push(stack.pop().unwrap())
                }
                expect_operand = false;
            }

            MathToken::COMMA => {
                if expect_operand {
                    return Err(ParseError::ExpectedOperand);
                }
                let mut found_lparen = false;
                while let Some(top) = stack.pop() {
                    if top.is_lparen() {
                        found_lparen = true;
                        break;
                    }
                    output.push(top);
                }
                stack.push(MathToken::LParen);
                if !found_lparen {
                    return Err(ParseError::MissingParen);
                }
                expect_operand = true;
            }

            MathToken::Operator(op) => {
                if op.is_unary() {
                    // unary operands have heighets precedence
                    if !expect_operand {
                        return Err(ParseError::UnexpectedOperand);
                    }
                    stack.push(MathToken::Operator(op));
                } else {
                    if expect_operand {
                        return Err(ParseError::ExpectedOperand);
                    }
                    let prec = op.precedence();
                    let right_assoc = op.is_righthand();

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

    if expect_operand {
        return Err(ParseError::ExpectedOperand);
    }

    while let Some(top) = stack.pop() {
        if top.is_paren() {
            return Err(ParseError::UnclosedParens);
        }
        output.push(top);
    }

    Ok(output)
}
