
#[derive(Debug)]
pub struct FunParam {
    pub(crate) tp: Box<Type>,
    pub(crate) name: String,
}

#[derive(Debug)]
pub struct FunType {
    /// the return type of this function
    pub(crate) ret: Box<Type>,
    /// the arguments passed to this function
    pub(crate) args: Vec<FunParam>,
}

#[derive(Debug)]
pub enum Type {
    /// type is unknown at compile time
    Unknown,
    /// unit type - basically void
    Unit,
    /// scalar boolean value
    Boolean,
    /// 1 byte character
    Char,
    /// 4 byte signed integer
    Int,
    /// 4 byte unsigned integer
    UInt,
    /// 8 byte signed integer
    Long,
    /// 8 byte unsigned integer
    ULong,
    /// Double precision integer
    Double,
    /// String type
    String,
    /// A reference type, provides indirection
    Reference(Box<Type>),
    /// an optional/nullable type
    Optional(Box<Type>),
    /// functional type
    Function(FunType),
    /// A sized array type
    Array(Box<Type>, usize),
    /// A variable sized, non-owning view of contiguous memory
    View(Box<Type>),
    /// user defined non-scalar type
    UserDefined(String),
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        match value {
            "()" => Self::Unit,
            "bool" => Self::Boolean,
            "char" => Self::Char,
            "int" => Self::Int,
            "uint" => Self::UInt,
            "long" => Self::Long,
            "ulong" => Self::ULong,
            "double" => Self::Double,
            "str" => Self::String,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug)]
pub enum VariableDeclarationMode {
    Const,
    Mutable,
}
