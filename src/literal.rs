
#[derive(Copy, Clone)]
pub enum LiteralKind {
    Unit,
    Boolean,
    Char,
    Int,
    Double,
    String
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LiteralRef<'input> {
    Unit,
    Boolean(bool),
    Char(char),
    Int(u64),
    Double(f64),
    String(&'input str),
}

#[derive(Clone,Debug, PartialEq)]
pub enum Literal {
    Unit,
    Boolean(bool),
    Char(char),
    Int(u64),
    Double(f64),
    String(String),
}

impl Eq for Literal {
}
