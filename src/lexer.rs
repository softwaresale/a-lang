mod input_reader;
pub mod token_stream;

#[cfg(test)]
mod test;

use std::error::Error;
use std::iter::Peekable;
use std::str::Chars;
use crate::error::source::SourceError;
use crate::input::SourceInput;
use crate::lexer::token_stream::TokenStream;
use crate::literal::{Literal};
use crate::location::{SourceLocation, SourceRange};
use crate::token::{Token, TokenKind};

pub struct Lexer<'input> {
    /// the input we are consuming
    input: Peekable<Chars<'input>>,
    /// the location in source we are currently at
    location: SourceLocation,
    /// buffer of tokens
    token_buf: Vec<Token>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input SourceInput) -> Self {
        Self {
            input: input.source().chars().peekable(),
            location: SourceLocation::default(),
            token_buf: Vec::new()
        }
    }

    /// read all available tokens into a token stream
    pub fn into_token_stream(mut self) -> Result<TokenStream, SourceError> {
        let mut tokens = Vec::<Token>::new();
        let mut found_eof = false;
        while !found_eof {
            let next_token = self.scan_next()?;
            match &next_token.kind {
                TokenKind::EOF => found_eof = true,
                _ => {}
            }

            tokens.push(next_token);
        }

        Ok(TokenStream::new(tokens))
    }

    fn decide_next(&mut self, check_char: char, on_true: TokenKind, on_false: TokenKind) -> TokenKind {
        if self.input.peek().is_some_and(|next_ch| *next_ch == check_char) {
            self.input.next();
            self.location.bump();
            on_true
        } else {
            on_false
        }
    }

    fn take_until(&mut self, stop_char: char) -> String {
        let mut buffer = String::new();
        while let Some(char_literal) = self.input.next_if(|ch| *ch != stop_char) {
            self.location.bump();
            buffer.push(char_literal);
        }

        buffer
    }

    fn take_numerical_literal(&mut self, first_char: char) -> Result<Literal, Box<dyn Error>> {
        let mut literal_buffer = String::from(first_char);
        let mut has_dot = false;
        while let Some(next_ch) = self.input.next_if(|next_ch| next_ch.is_numeric() || *next_ch == '.') {
            if next_ch == '.' {
                has_dot = true;
            }

            self.location.bump();
            literal_buffer.push(next_ch)
        }

        if has_dot {
            literal_buffer.parse::<f64>()
                .map(|lit| Literal::Double(lit))
                .map_err(|err| err.into())
        } else {
            literal_buffer.parse::<u64>()
                .map(|lit| Literal::Int(lit))
                .map_err(|err| err.into())
        }
    }

    fn take_identifier(&mut self, first_char: char) -> String {
        let mut identifier = String::from(first_char);
        while let Some(next_ch) = self.input.next_if(|next_ch| next_ch.is_alphanumeric() || *next_ch == '_') {
            self.location.bump();
            identifier.push(next_ch)
        }

        identifier
    }

    pub(crate) fn scan_next(&mut self) -> Result<Token, SourceError> {
        // each whitespace
        while let Some(next_ch) = self.input.next_if(|next_ch| next_ch.is_whitespace()) {
            self.location.update_with_char(next_ch);
        }

        let Some(next_ch) = self.input.next() else {
            return Ok(Token::eof());
        };

        let start = self.location.clone();
        self.location.bump();

        let matched_token = match next_ch {
            ',' => Ok(TokenKind::Comma),
            ';' => Ok(TokenKind::Semicolon),
            '(' => Ok(TokenKind::LParen),
            ')' => Ok(TokenKind::RParen),
            '[' => Ok(TokenKind::LBracket),
            ']' => Ok(TokenKind::RBracket),
            '{' => Ok(TokenKind::LBrace),
            '}' => Ok(TokenKind::RBrace),
            ':' => Ok(TokenKind::Colon),
            '+' => Ok(TokenKind::Plus),
            '-' => Ok(TokenKind::Minus),
            '*' => Ok(TokenKind::Times),
            '/' => Ok(TokenKind::Divides),
            '&' => Ok(self.decide_next('&', TokenKind::And, TokenKind::Ref)),
            '.' => Ok(TokenKind::Access),
            '=' => Ok(self.decide_next('=', TokenKind::Eq, TokenKind::Assign)),
            '!' => Ok(self.decide_next('=', TokenKind::Neq, TokenKind::Not)),
            '>' => Ok(self.decide_next('=', TokenKind::Lte, TokenKind::Lt)),
            '<' => Ok(self.decide_next('=', TokenKind::Gte, TokenKind::Gt)),
            '|' => {
                if self.input.peek().is_some_and(|next_ch| *next_ch == '|') {
                    self.input.next();
                    self.location.bump();
                    Ok(TokenKind::Or)
                } else {
                    // TODO this is bad and needs to be fixed...
                    Ok(TokenKind::Or)
                }
            }
            '?' => Ok(TokenKind::Nullable),
            '\'' => {
                // start taking a character literal
                let buffer = self.take_until('\'');

                let ending_char = self.input.next();
                if ending_char.is_some_and(|ch| ch == '\'') {
                    self.location.bump();
                    let end = self.location.clone();
                    let char_literal = match buffer.len() {
                        0 => Err(SourceError::new("Char literal cannot be empty", (start, end).into())),
                        1 => Ok(buffer.chars().nth(0).unwrap()),
                        _ => Err(SourceError::new("Char literal can only contain one character", (start, end).into()))
                    }?;

                    Ok(TokenKind::Lit(Literal::Char(char_literal)))
                } else {
                    Err(SourceError::new("Unterminated character literal", (start, self.location.clone()).into()))
                }
            }
            '"' => {
                // start taking a character literal
                let buffer = self.take_until('"');

                let ending_char = self.input.next();
                if ending_char.is_some_and(|ch| ch == '"') {
                    self.location.bump();
                    Ok(TokenKind::Lit(Literal::String(buffer)))
                } else {
                    Err(SourceError::new("Unterminated character literal", (start, self.location.clone()).into()))
                }
            }
            other => {
                if other.is_numeric() {
                    // try a digit literal
                    match self.take_numerical_literal(other) {
                        Ok(lit) => Ok(TokenKind::Lit(lit)),
                        Err(parse_error) => {
                            let end = self.location.clone();
                            Err(SourceError::new(format!("Invalid numerical literal: {}", parse_error), (start, end).into()))
                        }
                    }
                } else if other.is_alphabetic() || other == '_' {
                    // try an identifier
                    let ident = self.take_identifier(other);
                    match TokenKind::try_from(ident) {
                        Ok(matched_token) => Ok(matched_token),
                        Err(unmatched) => Ok(TokenKind::Ident(unmatched))
                    }
                } else {
                    // unidentified syntax
                    let end = self.location.clone();
                    Err(SourceError::new(format!("Unidentified character: {}", other), (start, end).into()))
                }
            }
        }?;

        let end = self.location.clone();
        let location = SourceRange { start, end };

        Ok(Token {
            kind: matched_token,
            location,
        })
    }
}
