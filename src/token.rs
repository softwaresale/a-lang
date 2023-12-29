
use crate::literal::{LiteralRef};
use crate::location::{HasLocation, SourceLocation, SourceRange};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind<'input> {
    EOF,
    /// identifier
    Ident(&'input str),
    /// literal value
    Lit(LiteralRef<'input>),

    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Colon,

    // Operators
    Plus,
    Minus,
    Times,
    Divides,
    Ref,
    Access,
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    Assign,
    And,
    Or,
    Not,

    // General keyword token
    FunDecl,
    ObjDecl,
    Composes,
    If,
    Else,
    While,
    For,
    In,
    Let,
    Const,
    Mut,
    Return,
    Break,
    As,

    // Other
    Nullable,
}

impl<'input> TryFrom<&'input str> for TokenKind<'input> {
    type Error = &'input str;

    fn try_from(value: &'input str) -> Result<Self, Self::Error> {
        match value {
            "," => Ok(TokenKind::Comma),
            ";" => Ok(TokenKind::Semicolon),
            "(" => Ok(TokenKind::LParen),
            ")" => Ok(TokenKind::RParen),
            "[" => Ok(TokenKind::LBracket),
            "]" => Ok(TokenKind::RBracket),
            "{" => Ok(TokenKind::LBrace),
            "}" => Ok(TokenKind::RBrace),
            ":" => Ok(TokenKind::Colon),
            "+" => Ok(TokenKind::Plus),
            "-" => Ok(TokenKind::Minus),
            "*" => Ok(TokenKind::Times),
            "/" => Ok(TokenKind::Divides),
            "&" => Ok(TokenKind::Ref),
            "." => Ok(TokenKind::Access),
            "==" => Ok(TokenKind::Eq),
            "!=" => Ok(TokenKind::Neq),
            ">" => Ok(TokenKind::Gt),
            ">=" => Ok(TokenKind::Gte),
            "<" => Ok(TokenKind::Lt),
            "<=" => Ok(TokenKind::Lte),
            "=" => Ok(TokenKind::Assign),
            "&&" => Ok(TokenKind::And),
            "||" => Ok(TokenKind::Or),
            "!" => Ok(TokenKind::Not),
            "?" => Ok(TokenKind::Nullable),
            "fun" => Ok(TokenKind::FunDecl),
            "object" => Ok(TokenKind::ObjDecl),
            "if" => Ok(TokenKind::If),
            "else" => Ok(TokenKind::Else),
            "while" => Ok(TokenKind::While),
            "for" => Ok(TokenKind::For),
            "in" => Ok(TokenKind::In),
            "let" => Ok(TokenKind::Let),
            "const" => Ok(TokenKind::Const),
            "mut" => Ok(TokenKind::Mut),
            "return" => Ok(TokenKind::Return),
            "break" => Ok(TokenKind::Break),
            "as" => Ok(TokenKind::As),
            "composes" => Ok(TokenKind::Composes),
            _ => Err(value)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Token<'input> {
    /// what kind of token this is
    pub(crate) kind: TokenKind<'input>,
    /// where the token occurs in terms of source
    pub(crate) location: SourceRange,
}

impl<'input> HasLocation for Token<'input> {
    fn source_range(&self) -> SourceRange {
        self.location
    }
}
