use crate::literal::{Literal};
use crate::location::SourceRange;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Keyword {
    /// "fun"
    FunDecl,
    /// "object"
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
}

impl TryFrom<&str> for Keyword {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "fun" => Ok(Keyword::FunDecl),
            "object" => Ok(Keyword::ObjDecl),
            "if" => Ok(Keyword::FunDecl),
            "else" => Ok(Keyword::Else),
            "while" => Ok(Keyword::While),
            "for" => Ok(Keyword::For),
            "in" => Ok(Keyword::In),
            "let" => Ok(Keyword::Let),
            "const" => Ok(Keyword::Const),
            "mut" => Ok(Keyword::Mut),
            "return" => Ok(Keyword::Return),
            "break" => Ok(Keyword::Break),
            _ => Err(())
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenKind {
    EOF,
    /// identifier
    Ident(String),
    /// literal value
    Lit(Literal),

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
    Keyword(Keyword),

    // Other
    Nullable,
}

impl TryFrom<&str> for TokenKind {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
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
            _ => Err(())
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    /// what kind of token this is
    pub(crate) kind: TokenKind,
    /// Where in the input this token occurs
    pub(crate) location: SourceRange,
}

impl Token {
    pub fn eof() -> Self {
        Self {
            kind: TokenKind::EOF,
            location: Default::default(),
        }
    }
}
