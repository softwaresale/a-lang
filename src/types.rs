use std::collections::HashMap;
use crate::literal::Literal;

#[derive(Debug, Clone)]
pub struct FunParam {
    pub(crate) tp: Box<Type>,
    pub(crate) name: String,
}

#[derive(Debug, Clone)]
pub struct FunType {
    /// the return type of this function
    pub(crate) ret: Box<Type>,
    /// the arguments passed to this function
    pub(crate) args: Vec<FunParam>,
}

#[derive(Debug, Clone)]
pub struct ObjectType {
    /// name of the object
    /// TODO this can probably be omitted
    pub(crate) name: String,
    /// the properties on this object
    pub(crate) props: HashMap<String, Box<Type>>,
    /// the composed types with this object
    /// TODO might need some reworking
    pub(crate) comps: HashMap<String, String>,
}

#[derive(Debug, Clone)]
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
    /// Object type
    Object(ObjectType),
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

#[derive(Debug, Clone)]
pub enum VariableDeclarationMode {
    Const,
    Mutable,
}

impl From<Literal> for Type {
    fn from(value: Literal) -> Self {
        match value {
            Literal::Unit => Type::Unit,
            // TODO not sure how to type this...
            Literal::Null => Type::Optional(Type::Unknown.into()),
            Literal::Boolean(_) => Type::Boolean,
            Literal::Char(_) => Type::Char,
            Literal::Int(_) => Type::UInt,
            Literal::Double(_) => Type::Double,
            Literal::String(_) => Type::String,
        }
    }
}
