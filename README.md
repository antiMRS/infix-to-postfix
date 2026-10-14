# Infix to Postfix Converter

A Rust crate that converts mathematical expressions from infix notation to postfix (Reverse Polish) notation using the shunting-yard algorithm.

The original algorithm was described by Edsger W. Dijkstra in 1961.  
[Original paper (PDF)](https://www.cs.utexas.edu/~EWD/MCReps/MR35.PDF)

## Features

- Generic over value type `V` and operator type `O`
- Supports binary and unary operators
- Handles operator precedence and associativity
- Supports parentheses and function calls with comma-separated arguments
- Provides ready-to-use default types: `DefaultValue`, `DefaultOperator`, `DefaultMathToken`
- Returns detailed `ParseError` variants on invalid input

## Installation

Write a:

    cargo add infix_to_postix

## Usage

### Basic arithmetic

```rust
use infix_to_postfix::defaults::*;
use infix_to_postfix::{into_postfix, MathToken};

let infix = vec![
    MathToken::Value(DefaultValue::Integer(1)),
    MathToken::Operator(DefaultOperator::Add),
    MathToken::Value(DefaultValue::Integer(2)),
];
let postfix = into_postfix(infix).unwrap();
assert_eq!(postfix, vec![
    MathToken::Value(DefaultValue::Integer(1)),
    MathToken::Value(DefaultValue::Integer(2)),
    MathToken::Operator(DefaultOperator::Add),
]);
```

### Operator precedence

```rust
use infix_to_postfix::defaults::*;
use infix_to_postfix::{into_postfix, MathToken};

let infix = vec![
    MathToken::Value(DefaultValue::Integer(1)),
    MathToken::Operator(DefaultOperator::Add),
    MathToken::Value(DefaultValue::Integer(2)),
    MathToken::Operator(DefaultOperator::Mul),
    MathToken::Value(DefaultValue::Integer(3)),
];
let postfix = into_postfix(infix).unwrap();
assert_eq!(postfix, vec![
    MathToken::Value(DefaultValue::Integer(1)),
    MathToken::Value(DefaultValue::Integer(2)),
    MathToken::Value(DefaultValue::Integer(3)),
    MathToken::Operator(DefaultOperator::Mul),
    MathToken::Operator(DefaultOperator::Add),
]);
```

### Parentheses and unary operators

```rust
use infix_to_postfix::defaults::*;
use infix_to_postfix::{into_postfix, MathToken};

let infix = vec![
    MathToken::Operator(DefaultOperator::Neg),
    MathToken::LParen,
    MathToken::Value(DefaultValue::Integer(1)),
    MathToken::Operator(DefaultOperator::Add),
    MathToken::Value(DefaultValue::Integer(2)),
    MathToken::RParen,
];
let postfix = into_postfix(infix).unwrap();
assert_eq!(postfix, vec![
    MathToken::Value(DefaultValue::Integer(1)),
    MathToken::Value(DefaultValue::Integer(2)),
    MathToken::Operator(DefaultOperator::Add),
    MathToken::Operator(DefaultOperator::Neg),
]);
```

## Custom operators

To use your own operator type, implement the `Operator` trait:

```rust
use infix_to_postfix::Operator;

enum MyOp {
    Plus,
    Minus,
    Not,
}

impl Operator for MyOp {
    fn precedence(&self) -> u8 {
        match self {
            MyOp::Plus | MyOp::Minus => 1,
            MyOp::Not => 2,
        }
    }

    fn is_righthand(&self) -> bool {
        false
    }

    fn is_binary(&self) -> bool {
        match self {
            MyOp::Not => false,
            _ => true,
        }
    }
}
```

## Errors

The `into_postfix` function returns a `Result<Vec<MathToken<V, O>>, ParseError>`.  
Possible errors are:

- `IncorrectMathSeq` – invalid mathematical sequence
- `MissingParen` – closing parenthesis or comma without matching opening parenthesis
- `UnclosedParens` – opening parenthesis without matching closing parenthesis
- `UnexpectedOperand` – operand encountered when an operator was expected
- `ExpectedOperand` – operand expected but a different token was encountered

## License

Licensed under the MIT license. See the `LICENSE` file for details.
