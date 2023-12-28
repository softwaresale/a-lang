
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

impl<'input> Into<Literal> for LiteralRef<'input> {
    fn into(self) -> Literal {
        match self {
            LiteralRef::Unit => Literal::Unit,
            LiteralRef::Boolean(bool) => Literal::Boolean(bool),
            LiteralRef::Char(ch) => Literal::Char(ch),
            LiteralRef::Int(int) => Literal::Int(int),
            LiteralRef::Double(db) => Literal::Double(db),
            LiteralRef::String(str_ref) => Literal::String(str_ref.to_string())
        }
    }
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
