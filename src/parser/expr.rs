use crate::ast::{Ast, IdentNode, LitNode};
use crate::error::source::SourceError;
use crate::parser::{Parser, ParseResult};
use crate::token::TokenKind;

/// The expression parser

impl<'input> Parser<'input> {
    /// <expr> ::= TODO
    pub(crate) fn parse_expr(&mut self) -> ParseResult {
        self.parse_atom()
    }

    /// <atom> ::= <literal> | <ident> | <parens_expr> | <fun_call> | <array_access>
    pub(crate) fn parse_atom(&mut self) -> ParseResult {
        self.one_of([
            Self::parse_parens_expr,
            Self::parse_fun_call,
            Self::parse_literal,
            Self::parse_ident
        ])
    }

    /// <array_access> ::= <ident> '\[' <expr> '\]'
    fn parse_array_access(&mut self) -> ParseResult {
        let ident = self.parse_ident()?;
    }

    /// <parens_result> ::= '(' <expr> ')'
    fn parse_parens_expr(&mut self) -> ParseResult {
        self.tokens.accept(TokenKind::LParen)
            .map_err(|err| vec![err])?;
        let expr = self.parse_expr()?;
        self.tokens.accept(TokenKind::RParen)
            .map_err(|err| vec![err])?;

        Ok(expr)
    }

    fn parse_literal(&mut self) -> ParseResult {
        self.tokens.accept_if_map(|token| {
            match token.kind {
                TokenKind::Lit(lit_ref) => {
                    Ok(Ast::Literal(LitNode {
                        lit: lit_ref.into(),
                        location: token.location
                    }).into())
                }
                other => {
                    Err(SourceError::new(format!("Expected identifier but got '{:?}' instead", other), token.location))
                }
            }
        })
            .map_err(|err| err.into())
    }

    pub(crate) fn parse_ident(&mut self) -> ParseResult {
        self.tokens.accept_if_map(|token| {
            match token.kind {
                TokenKind::Ident(ident) => {
                    Ok(Ast::Identifier(IdentNode {
                        ident: ident.to_string(),
                        location: token.location
                    }).into())
                }
                other => {
                    Err(SourceError::new(format!("Expected identifier but got '{:?}' instead", other), token.location))
                }
            }
        })
            .map_err(|err| err.into())
    }
}