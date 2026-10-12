use crate::Operator;
use core::{cmp::PartialEq, fmt};

pub enum MathToken<V, O: Operator> {
    Value(V),
    Operator(O),
    Call(V),
    RParen,
    LParen,
    COMMA,
}

impl<V, O: Operator> MathToken<V, O> {
    pub fn as_value(&self) -> Option<&V> {
        if let Self::Value(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_operator(&self) -> Option<&O> {
        if let Self::Operator(o) = self {
            Some(o)
        } else {
            None
        }
    }

    pub fn is_value(&self) -> bool {
        self.as_value().is_some()
    }

    pub fn is_operator(&self) -> bool {
        self.as_operator().is_some()
    }

    pub fn is_lparen(&self) -> bool {
        if let Self::LParen = self { true } else { false }
    }

    pub fn is_rparen(&self) -> bool {
        if let Self::RParen = self { true } else { false }
    }

    pub fn is_paren(&self) -> bool {
        match self {
            Self::RParen | Self::LParen => true,
            _ => false,
        }
    }
}

impl<V, O> fmt::Debug for MathToken<V, O>
where
    V: fmt::Debug,
    O: Operator + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Value(v) => f.debug_tuple("V").field(v).finish(),
            Self::Operator(o) => f.debug_tuple("O").field(o).finish(),
            Self::RParen => write!(f, ")"),
            Self::LParen => write!(f, "("),
            Self::COMMA => write!(f, ","),
            Self::Call(v) => f.debug_tuple("C").field(v).finish(),
        }
    }
}

impl<V, O> PartialEq for MathToken<V, O>
where
    V: PartialEq,
    O: Operator + PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Self::Value(a), Self::Value(b)) => a == b,
            (Self::Operator(a), Self::Operator(b)) => a == b,
            (Self::Call(a), Self::Call(b)) => a == b,
            (Self::RParen, Self::RParen)
            | (Self::LParen, Self::LParen)
            | (Self::COMMA, Self::COMMA) => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DefaultOperator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Neg,
    // comparsion
    Lt,
    LtEq,
    Gt,
    GtEq,
    Eq,
    NeEq,
    // logics
    And,
    Or,
    Ne,
    // bit
    BAnd,
    BOr,
    BXor,
    BNe,
    LShift,
    RShift,
}

impl Operator for DefaultOperator {
    fn precedence(&self) -> u8 {
        match self {
            Self::Or => 1,
            Self::And => 2,
            Self::Eq | Self::NeEq => 3,
            Self::Lt | Self::LtEq | Self::Gt | Self::GtEq => 4,
            Self::BOr => 5,
            Self::BXor => 6,
            Self::BAnd => 7,
            Self::LShift | Self::RShift => 8,
            Self::Add | Self::Sub => 9,
            Self::Mul | Self::Div | Self::Rem => 10,
            Self::Neg | Self::Ne | Self::BNe => 11,
        }
    }

    fn is_righthand(&self) -> bool {
        match self {
            Self::LShift | Self::RShift | Self::Sub => false,
            _ => true,
        }
    }

    fn is_binary(&self) -> bool {
        match self {
            Self::Neg | Self::Ne | Self::BNe => false,
            _ => true,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DefaultValue {
    Integer(i32),
    Float(f32),
}

impl Default for DefaultValue {
    fn default() -> Self {
        DefaultValue::Integer(i32::default())
    }
}

pub type DefaultMathToken = MathToken<DefaultValue, DefaultOperator>;
