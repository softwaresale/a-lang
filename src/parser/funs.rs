use crate::ast::{Ast, FunCallNode, NamedArgNode};
use crate::error::parse::ParseErr;
use crate::location::SourceRange;
use crate::parser::{Parser, ParseResult};
use crate::token::TokenKind;

/// parsers related to functions

impl<'input> Parser<'input> {

    /// <fun_call> ::= <ident> '(' <args> ')'
    pub(crate) fn parse_fun_call(&mut self) -> ParseResult {
        let ident = self.parse_ident()
            .map_err(|err| err.into_non_fatal())?;
        self.tokens.accept(TokenKind::LParen)
            .map_err(|err| ParseErr::NonFatal(err))?;

        let args = self.parse_repeated(Self::parse_arg, TokenKind::Comma, TokenKind::RParen)?;
        let end_paren = self.tokens.accept(TokenKind::RParen)
            .map_err(|err| ParseErr::Fatal(err))?;
        let loc = SourceRange::spanned(ident.as_ref(), &end_paren.location);
        Ok(Ast::FunCall(FunCallNode {
            fun_name: ident,
            args,
            location: loc,
        }).into())
    }

    /// <arg> ::= <expr> | <ident> '=' <expr>
    fn parse_arg(&mut self) -> ParseResult {
        self.one_of([
            Self::parse_named_arg,
            Self::parse_expr
        ])
    }

    fn parse_named_arg(&mut self) -> ParseResult {
        let named_arg = self.parse_ident()?;
        self.tokens.accept(TokenKind::Assign)
            .map_err(|err| ParseErr::NonFatal(err))?;

        let value = self.parse_expr()
            .map_err(|err| err.into_fatal())?;
        let loc = SourceRange::spanned(named_arg.as_ref(), value.as_ref());

        Ok(Ast::NamedArg(NamedArgNode {
            param_name: named_arg,
            value,
            location: loc,
        }).into())
    }
}
