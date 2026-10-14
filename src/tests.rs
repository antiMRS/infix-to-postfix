use super::*;
use crate::math_token::{DefaultMathToken, DefaultOperator, DefaultValue};
use MathToken::{LParen, Operator as Op, RParen, Value};

fn v(i: i32) -> DefaultMathToken {
    Value(DefaultValue::Integer(i))
}

fn assert_rpn(input: Vec<DefaultMathToken>, expected: Vec<DefaultMathToken>) {
    let result = into_postfix(input).expect("expected Ok");
    assert_eq!(result, expected, "Postfix notation mismatch");
}

fn assert_err(input: Vec<DefaultMathToken>, expect: ParseError) {
    if let Err(e) = into_postfix(input) {
        if e != expect {
            panic!("expected {expect:?} got: {e:?}")
        }
    } else {
        panic!("excpect Err got Ok")
    }
}

#[test]
fn single_value() {
    assert_rpn(vec![v(1)], vec![v(1)]);
}

#[test]
fn simple_add() {
    assert_rpn(
        vec![v(1), Op(DefaultOperator::Add), v(2)],
        vec![v(1), v(2), Op(DefaultOperator::Add)],
    );
}

#[test]
fn mul_has_higher_precedence_than_add() {
    // 1 + 2 * 3
    assert_rpn(
        vec![
            v(1),
            Op(DefaultOperator::Add),
            v(2),
            Op(DefaultOperator::Mul),
            v(3),
        ],
        vec![
            v(1),
            v(2),
            v(3),
            Op(DefaultOperator::Mul),
            Op(DefaultOperator::Add),
        ],
    );
}

#[test]
fn parens_override_precedence() {
    // ( 1 + 2 ) * 3
    assert_rpn(
        vec![
            LParen,
            v(1),
            Op(DefaultOperator::Add),
            v(2),
            RParen,
            Op(DefaultOperator::Mul),
            v(3),
        ],
        vec![
            v(1),
            v(2),
            Op(DefaultOperator::Add),
            v(3),
            Op(DefaultOperator::Mul),
        ],
    );
}

#[test]
fn left_associativity_of_sub() {
    // 1 - 2 - 3 == (1 - 2) - 3
    assert_rpn(
        vec![
            v(1),
            Op(DefaultOperator::Sub),
            v(2),
            Op(DefaultOperator::Sub),
            v(3),
        ],
        vec![
            v(1),
            v(2),
            Op(DefaultOperator::Sub),
            v(3),
            Op(DefaultOperator::Sub),
        ],
    );
}

#[test]
fn unary_minus_simple() {
    // -1
    assert_rpn(
        vec![Op(DefaultOperator::Neg), v(1)],
        vec![v(1), Op(DefaultOperator::Neg)],
    );
}

#[test]
fn unary_minus_binds_tighter_than_mul() {
    // -1 * 2
    assert_rpn(
        vec![
            Op(DefaultOperator::Neg),
            v(1),
            Op(DefaultOperator::Mul),
            v(2),
        ],
        vec![
            v(1),
            Op(DefaultOperator::Neg),
            v(2),
            Op(DefaultOperator::Mul),
        ],
    );
}

#[test]
fn unary_minus_after_binary_op() {
    // 1 + -2
    assert_rpn(
        vec![
            v(1),
            Op(DefaultOperator::Add),
            Op(DefaultOperator::Neg),
            v(2),
        ],
        vec![
            v(1),
            v(2),
            Op(DefaultOperator::Neg),
            Op(DefaultOperator::Add),
        ],
    );
}

#[test]
fn double_unary_minus() {
    // -- 1
    assert_rpn(
        vec![Op(DefaultOperator::Neg), Op(DefaultOperator::Neg), v(1)],
        vec![v(1), Op(DefaultOperator::Neg), Op(DefaultOperator::Neg)],
    );
}

#[test]
fn unary_minus_over_parens() {
    // -(1 + 2)
    assert_rpn(
        vec![
            Op(DefaultOperator::Neg),
            LParen,
            v(1),
            Op(DefaultOperator::Add),
            v(2),
            RParen,
        ],
        vec![
            v(1),
            v(2),
            Op(DefaultOperator::Add),
            Op(DefaultOperator::Neg),
        ],
    );
}

#[test]
fn err_leading_binary_op() {
    assert_err(
        vec![Op(DefaultOperator::Add), v(1)],
        ParseError::ExpectedOperand,
    );
}

#[test]
fn err_dangling_operator() {
    assert_err(
        vec![v(1), Op(DefaultOperator::Add)],
        ParseError::ExpectedOperand,
    );
}

#[test]
fn err_dangling_unary_operator() {
    assert_err(
        vec![v(1), Op(DefaultOperator::Add), Op(DefaultOperator::Neg)],
        ParseError::ExpectedOperand,
    );
}

#[test]
fn err_two_values_in_a_row() {
    assert_err(vec![v(1), v(2)], ParseError::UnexpectedOperand);
}

#[test]
fn err_two_binary_ops_in_a_row() {
    assert_err(
        vec![Op(DefaultOperator::Add), Op(DefaultOperator::Mul), v(1)],
        ParseError::ExpectedOperand,
    );
}

#[test]
fn err_unbalanced_open_paren() {
    assert_err(
        vec![LParen, v(1), Op(DefaultOperator::Add), v(2)],
        ParseError::UnclosedParens,
    );
}

#[test]
fn err_unbalanced_close_paren() {
    assert_err(vec![v(1), RParen], ParseError::MissingParen);
}

#[test]
fn err_empty_parens() {
    assert_err(vec![LParen, RParen], ParseError::ExpectedOperand);
}

#[test]
fn err_unary_before_close_paren() {
    assert_err(
        vec![LParen, v(1), Op(DefaultOperator::Neg), RParen],
        ParseError::UnexpectedOperand,
    );
}

#[test]
fn simple_and() {
    // a && b
    assert_rpn(
        vec![v(1), Op(DefaultOperator::And), v(2)],
        vec![v(1), v(2), Op(DefaultOperator::And)],
    );
}

#[test]
fn simple_or() {
    // a || b
    assert_rpn(
        vec![v(1), Op(DefaultOperator::Or), v(2)],
        vec![v(1), v(2), Op(DefaultOperator::Or)],
    );
}

#[test]
fn simple_not() {
    // !a — unary
    assert_rpn(
        vec![Op(DefaultOperator::Ne), v(1)],
        vec![v(1), Op(DefaultOperator::Ne)],
    );
}

#[test]
fn and_has_higher_precedence_than_or() {
    // a || b && c  ==  a || (b && c)
    assert_rpn(
        vec![
            v(1),
            Op(DefaultOperator::Or),
            v(2),
            Op(DefaultOperator::And),
            v(3),
        ],
        vec![
            v(1),
            v(2),
            v(3),
            Op(DefaultOperator::And),
            Op(DefaultOperator::Or),
        ],
    );
}

#[test]
fn or_higher_than_equality() {
    // a == b || c  ==  (a == b) || c  (|| prec = 1, == prec = 3)
    // Значит наоборот: == выше ||, то есть a == (b || c) не получится,
    // а получится (a == b) || c.
    assert_rpn(
        vec![
            v(1),
            Op(DefaultOperator::Eq),
            v(2),
            Op(DefaultOperator::Or),
            v(3),
        ],
        vec![
            v(1),
            v(2),
            Op(DefaultOperator::Eq),
            v(3),
            Op(DefaultOperator::Or),
        ],
    );
}

#[test]
fn and_higher_than_equality() {
    // a == b && c  ==  a == (b && c)?
    // && prec = 2, == prec = 3 → == снимается раньше, значит
    // (a == b) && c.
    assert_rpn(
        vec![
            v(1),
            Op(DefaultOperator::Eq),
            v(2),
            Op(DefaultOperator::And),
            v(3),
        ],
        vec![
            v(1),
            v(2),
            Op(DefaultOperator::Eq),
            v(3),
            Op(DefaultOperator::And),
        ],
    );
}

#[test]
fn double_not() {
    // !!a
    assert_rpn(
        vec![Op(DefaultOperator::Ne), Op(DefaultOperator::Ne), v(1)],
        vec![v(1), Op(DefaultOperator::Ne), Op(DefaultOperator::Ne)],
    );
}

#[test]
fn not_binds_tighter_than_and() {
    // !a && b  ==  (!a) && b
    assert_rpn(
        vec![
            Op(DefaultOperator::Ne),
            v(1),
            Op(DefaultOperator::And),
            v(2),
        ],
        vec![
            v(1),
            Op(DefaultOperator::Ne),
            v(2),
            Op(DefaultOperator::And),
        ],
    );
}

#[test]
fn parens_override_logical_precedence() {
    // (a || b) && c
    assert_rpn(
        vec![
            LParen,
            v(1),
            Op(DefaultOperator::Or),
            v(2),
            RParen,
            Op(DefaultOperator::And),
            v(3),
        ],
        vec![
            v(1),
            v(2),
            Op(DefaultOperator::Or),
            v(3),
            Op(DefaultOperator::And),
        ],
    );
}

#[test]
fn err_dangling_not() {
    assert_err(
        vec![v(1), Op(DefaultOperator::Or), Op(DefaultOperator::Ne)],
        ParseError::ExpectedOperand,
    );
}

#[test]
fn err_leading_and() {
    assert_err(
        vec![Op(DefaultOperator::And), v(1)],
        ParseError::ExpectedOperand,
    );
}

#[test]
fn simple_rem() {
    assert_rpn(
        vec![v(7), Op(DefaultOperator::Rem), v(3)],
        vec![v(7), v(3), Op(DefaultOperator::Rem)],
    );
}

#[test]
fn rem_same_precedence_as_mul_and_div() {
    // a * b % c same as (a * b) % c
    assert_rpn(
        vec![
            v(1),
            Op(DefaultOperator::Mul),
            v(2),
            Op(DefaultOperator::Rem),
            v(3),
        ],
        vec![
            v(1),
            v(2),
            v(3),
            Op(DefaultOperator::Rem),
            Op(DefaultOperator::Mul),
        ],
    );
}

#[test]
fn rem_higher_than_add() {
    // a + b % c  ==  a + (b % c)
    assert_rpn(
        vec![
            v(1),
            Op(DefaultOperator::Add),
            v(2),
            Op(DefaultOperator::Rem),
            v(3),
        ],
        vec![
            v(1),
            v(2),
            v(3),
            Op(DefaultOperator::Rem),
            Op(DefaultOperator::Add),
        ],
    );
}

#[test]
fn parens_override_rem_precedence() {
    // (a + b) % c
    assert_rpn(
        vec![
            LParen,
            v(1),
            Op(DefaultOperator::Add),
            v(2),
            RParen,
            Op(DefaultOperator::Rem),
            v(3),
        ],
        vec![
            v(1),
            v(2),
            Op(DefaultOperator::Add),
            v(3),
            Op(DefaultOperator::Rem),
        ],
    );
}

#[test]
fn err_rem_missing_right_operand() {
    assert_err(
        vec![v(1), Op(DefaultOperator::Rem)],
        ParseError::ExpectedOperand,
    );
}

#[test]
fn err_rem_missing_left_operand() {
    assert_err(
        vec![Op(DefaultOperator::Rem), v(1)],
        ParseError::ExpectedOperand,
    );
}

#[test]
fn simple_band() {
    assert_rpn(
        vec![v(1), Op(DefaultOperator::BAnd), v(2)],
        vec![v(1), v(2), Op(DefaultOperator::BAnd)],
    );
}

#[test]
fn simple_bor() {
    assert_rpn(
        vec![v(1), Op(DefaultOperator::BOr), v(2)],
        vec![v(1), v(2), Op(DefaultOperator::BOr)],
    );
}

#[test]
fn simple_bxor() {
    assert_rpn(
        vec![v(1), Op(DefaultOperator::BXor), v(2)],
        vec![v(1), v(2), Op(DefaultOperator::BXor)],
    );
}

#[test]
fn simple_bnot() {
    // ~a — унарный
    assert_rpn(
        vec![Op(DefaultOperator::BNe), v(1)],
        vec![v(1), Op(DefaultOperator::BNe)],
    );
}

#[test]
fn simple_lshift() {
    assert_rpn(
        vec![v(1), Op(DefaultOperator::LShift), v(2)],
        vec![v(1), v(2), Op(DefaultOperator::LShift)],
    );
}

#[test]
fn simple_rshift() {
    assert_rpn(
        vec![v(1), Op(DefaultOperator::RShift), v(2)],
        vec![v(1), v(2), Op(DefaultOperator::RShift)],
    );
}

#[test]
fn bop_precedence_order() {
    // a | b ^ c & d
    // | prec = 5, ^ prec = 6, & prec = 7 → a | (b ^ (c & d))
    assert_rpn(
        vec![
            v(1),
            Op(DefaultOperator::BOr),
            v(2),
            Op(DefaultOperator::BXor),
            v(3),
            Op(DefaultOperator::BAnd),
            v(4),
        ],
        vec![
            v(1),
            v(2),
            v(3),
            v(4),
            Op(DefaultOperator::BAnd),
            Op(DefaultOperator::BXor),
            Op(DefaultOperator::BOr),
        ],
    );
}

#[test]
fn bit_ops_higher_than_logical_ops() {
    // a && b | c  →  && prec = 2, | prec = 5 → a && (b | c)
    assert_rpn(
        vec![
            v(1),
            Op(DefaultOperator::And),
            v(2),
            Op(DefaultOperator::BOr),
            v(3),
        ],
        vec![
            v(1),
            v(2),
            v(3),
            Op(DefaultOperator::BOr),
            Op(DefaultOperator::And),
        ],
    );
}

#[test]
fn shifts_higher_than_band() {
    // a & b << c  →  & prec = 7, << prec = 8 → a & (b << c)
    assert_rpn(
        vec![
            v(1),
            Op(DefaultOperator::BAnd),
            v(2),
            Op(DefaultOperator::LShift),
            v(3),
        ],
        vec![
            v(1),
            v(2),
            v(3),
            Op(DefaultOperator::LShift),
            Op(DefaultOperator::BAnd),
        ],
    );
}

#[test]
fn shifts_lower_than_add() {
    // a + b << c  →  + prec = 9, << prec = 8 → (a + b) << c
    assert_rpn(
        vec![
            v(1),
            Op(DefaultOperator::Add),
            v(2),
            Op(DefaultOperator::LShift),
            v(3),
        ],
        vec![
            v(1),
            v(2),
            Op(DefaultOperator::Add),
            v(3),
            Op(DefaultOperator::LShift),
        ],
    );
}

#[test]
fn left_associativity_of_shifts() {
    // a << b << c  ==  (a << b) << c
    assert_rpn(
        vec![
            v(1),
            Op(DefaultOperator::LShift),
            v(2),
            Op(DefaultOperator::LShift),
            v(3),
        ],
        vec![
            v(1),
            v(2),
            Op(DefaultOperator::LShift),
            v(3),
            Op(DefaultOperator::LShift),
        ],
    );
}

#[test]
fn bnot_binds_tighter_than_band() {
    // ~a & b  ==  (~a) & b
    assert_rpn(
        vec![
            Op(DefaultOperator::BNe),
            v(1),
            Op(DefaultOperator::BAnd),
            v(2),
        ],
        vec![
            v(1),
            Op(DefaultOperator::BNe),
            v(2),
            Op(DefaultOperator::BAnd),
        ],
    );
}

#[test]
fn parens_override_bit_ops() {
    // (a | b) & c
    assert_rpn(
        vec![
            LParen,
            v(1),
            Op(DefaultOperator::BOr),
            v(2),
            RParen,
            Op(DefaultOperator::BAnd),
            v(3),
        ],
        vec![
            v(1),
            v(2),
            Op(DefaultOperator::BOr),
            v(3),
            Op(DefaultOperator::BAnd),
        ],
    );
}

#[test]
fn err_lshift_missing_right() {
    assert_err(
        vec![v(1), Op(DefaultOperator::LShift)],
        ParseError::ExpectedOperand,
    );
}

#[test]
fn err_rshift_missing_left() {
    assert_err(
        vec![Op(DefaultOperator::RShift), v(1)],
        ParseError::ExpectedOperand,
    );
}

#[test]
fn mixed_arithmetic_bitwise() {
    // a % b + c << d
    // % и + (9/10) → (a % b); then (a%b) + c; then << d (8) —
    // но + (9) выше << (8), then ((a%b) + c) << d
    assert_rpn(
        vec![
            v(1),
            Op(DefaultOperator::Rem),
            v(2),
            Op(DefaultOperator::Add),
            v(3),
            Op(DefaultOperator::LShift),
            v(4),
        ],
        vec![
            v(1),
            v(2),
            Op(DefaultOperator::Rem),
            v(3),
            Op(DefaultOperator::Add),
            v(4),
            Op(DefaultOperator::LShift),
        ],
    );
}

#[test]
fn mixed_logic_bitwise_rem() {
    // a && b % c | d
    // % (10) > | (5) > && (2)
    // → a && ((b % c) | d)
    assert_rpn(
        vec![
            v(1),
            Op(DefaultOperator::And),
            v(2),
            Op(DefaultOperator::Rem),
            v(3),
            Op(DefaultOperator::BOr),
            v(4),
        ],
        vec![
            v(1),
            v(2),
            v(3),
            Op(DefaultOperator::Rem),
            v(4),
            Op(DefaultOperator::BOr),
            Op(DefaultOperator::And),
        ],
    );
}

#[test]
fn mixed_with_unary_and_parens() {
    // !(a & ~b) || c % d
    assert_rpn(
        vec![
            Op(DefaultOperator::Ne),
            LParen,
            v(1),
            Op(DefaultOperator::BAnd),
            Op(DefaultOperator::BNe),
            v(2),
            RParen,
            Op(DefaultOperator::Or),
            v(3),
            Op(DefaultOperator::Rem),
            v(4),
        ],
        vec![
            v(1),
            v(2),
            Op(DefaultOperator::BNe),
            Op(DefaultOperator::BAnd),
            Op(DefaultOperator::Ne),
            v(3),
            v(4),
            Op(DefaultOperator::Rem),
            Op(DefaultOperator::Or),
        ],
    );
}

#[test]
fn simple_call_test() {
    assert_rpn(
        vec![
            v(10),
            Op(DefaultOperator::Add),
            MathToken::Call(DefaultValue::Integer(5)),
            LParen,
            v(34),
            Op(DefaultOperator::Add),
            v(6),
            MathToken::COMMA,
            v(63),
            RParen,
        ],
        vec![
            v(10),
            v(34),
            v(6),
            Op(DefaultOperator::Add),
            v(63),
            MathToken::Call(DefaultValue::Integer(5)),
            Op(DefaultOperator::Add),
        ],
    );
}
