use crate::error::source::SourceError;
use crate::token::{Token, TokenKind};

#[derive(Debug, Eq, PartialEq)]
pub enum UnaryOp {
    /// bitwise negation
    BitNeg,
    /// boolean negation
    Not,
    /// numerical negation
    Neg,
    /// deference a pointer type
    Deref,
    /// reference a value
    Ref,
}

#[derive(Debug, Eq, PartialEq)]
pub enum BinaryOp {
    // Arith
    Plus,
    Minus,
    Times,
    Divides,
    Exp,
    // Comparisons
    Gt,
    Lt,
    Gte,
    Lte,
    Eq,
    Neq,
    // boolean operators
    And,
    Or,
    // access
    Access,
    ChainedAccess,
}

impl<'input> TryFrom<Token<'input>> for BinaryOp {
    type Error = SourceError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value.kind {
            TokenKind::Plus => Ok(BinaryOp::Plus),
            TokenKind::Minus => Ok(BinaryOp::Minus),
            TokenKind::Times => Ok(BinaryOp::Times),
            TokenKind::Divides => Ok(BinaryOp::Divides),
            TokenKind::Access => Ok(BinaryOp::Access),
            TokenKind::Eq => Ok(BinaryOp::Eq),
            TokenKind::Neq => Ok(BinaryOp::Neq),
            TokenKind::Gt => Ok(BinaryOp::Gt),
            TokenKind::Gte => Ok(BinaryOp::Gte),
            TokenKind::Lt => Ok(BinaryOp::Lt),
            TokenKind::Lte => Ok(BinaryOp::Lte),
            TokenKind::And => Ok(BinaryOp::And),
            TokenKind::Or => Ok(BinaryOp::Or),
            other => Err(SourceError::new(format!("'{:?}' is not a binary operator", other), value.location))
        }
    }
}

impl BinaryOp {
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOp::Plus |
            BinaryOp::Minus => 10,
            BinaryOp::Times |
            BinaryOp::Divides => 11,
            BinaryOp::Exp => 12,
            BinaryOp::Gt |
            BinaryOp::Lt |
            BinaryOp::Gte |
            BinaryOp::Lte => 9,
            BinaryOp::Eq |
            BinaryOp::Neq => 8,
            BinaryOp::And => 7,
            BinaryOp::Or => 6,
            BinaryOp::Access => 20,
            BinaryOp::ChainedAccess => 20
        }
    }
}
