use crate::frontend::ast::{Ast, FunCallNode, FunctionDeclarationNode, NamedArgNode, ParamNode, TypeSpecNode};
use crate::error::parse::ParseErr;
use crate::frontend::location::SourceRange;
use crate::frontend::parser::{Parser, ParseResult};
use crate::frontend::token::{TokenKind};
use crate::types::Type;

/// parsers related to functions

impl<'input> Parser<'input> {

    /// <fun_defn> ::= "fun" <ident> "(" <param_list> ")" <ret_type>? <block>
    pub(crate) fn parse_fun_defn(&mut self) -> ParseResult {
        let start_loc = self.tokens.accept(TokenKind::FunDecl)
            .map(|tok| tok.location)
            .map_err(|err| ParseErr::NonFatal(err))?;

        let ident = self.parse_ident()
            .map_err(|err| err.into_fatal())?;

        self.tokens.accept(TokenKind::LParen)
            .map_err(|err| ParseErr::Fatal(err))?;

        let params = self.parse_repeated(Self::parse_param, TokenKind::Comma, TokenKind::RParen)?;

        self.tokens.accept(TokenKind::RParen)
            .map_err(|err| ParseErr::Fatal(err))?;

        // parse an optional return type
        let ret_type = if self.tokens.check_next(|tok| match tok.kind {
            TokenKind::Colon => true,
            _ => false
        }) {
            self.tokens.accept(TokenKind::Colon)
                .map_err(|err| ParseErr::Fatal(err))?;
            self.parse_type_spec()
        } else {
            Ok(Ast::TypeSpec(TypeSpecNode {
                tp: Type::Unit,
                location: Default::default(),
            }).into())
        }
            .map_err(|err| err.into_fatal())?;

        let body = self.parse_block()
            .map_err(|err| err.into_fatal())?;

        let loc = SourceRange::spanned(&start_loc, body.as_ref());

        Ok(Ast::FunctionDeclaration(FunctionDeclarationNode {
            name: ident,
            params,
            ret_tp: ret_type,
            body,
            location: loc,
        }).into())
    }

    // <param> ::= <ident> <type_spec>
    fn parse_param(&mut self) -> ParseResult {
        let param_name = self.parse_ident()
            .map_err(|err| err.into_fatal())?;

        self.tokens.accept(TokenKind::Colon)
            .map_err(|err| ParseErr::Fatal(err))?;

        let type_spec = self.parse_type_spec()?;

        let loc = SourceRange::spanned(param_name.as_ref(), type_spec.as_ref());

        Ok(Ast::Param(ParamNode {
            name: param_name,
            tp: type_spec,
            location: loc,
        }).into())
    }

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
