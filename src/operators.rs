
#[derive(Debug)]
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

#[derive(Debug)]
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
