use crate::frontend::ast::{Ast, CompositionSpecNode, FieldDeclarationNode, ObjectDeclarationNode};
use crate::error::parse::ParseErr;
use crate::frontend::location::{HasLocation, SourceRange};
use crate::frontend::parser::{Parser, ParseResult};
use crate::frontend::token::TokenKind;

impl<'input> Parser<'input> {

    /// <object_decl> ::= "object" <ident> <compose_spec>? "{" <prop> "}"
    pub(crate) fn parse_object_decl(&mut self) -> ParseResult {
        let start_loc = self.tokens.accept(TokenKind::ObjDecl)
            .map(|tok| tok.location)
            .map_err(|err| ParseErr::NonFatal(err))?;

        let obj_name = self.parse_ident()
            .map_err(|err| err.into_fatal())?;

        let comp_specs = self.parse_composition_specs()?;

        self.tokens.accept(TokenKind::LBrace)
            .map_err(|err| ParseErr::Fatal(err))?;

        let props = self.parse_repeated(Self::parse_prop, TokenKind::Semicolon, TokenKind::RBrace)?;
        let end_loc = self.tokens.accept(TokenKind::RBrace)
            .map(|tok| tok.location)
            .map_err(|err| ParseErr::Fatal(err))?;

        let loc = SourceRange::spanned(&start_loc, &end_loc);

        Ok(Ast::ObjectDeclaration(ObjectDeclarationNode {
            name: obj_name,
            composition_specs: comp_specs,
            fields: props,
            location: loc,
        }).into())
    }

    fn parse_composition_specs(&mut self) -> Result<Vec<Box<Ast>>, ParseErr> {
        if self.tokens.check_next(|tok| tok.kind == TokenKind::Composes) {
            self.tokens.accept(TokenKind::Composes)
                .map_err(|err| ParseErr::Fatal(err))?;
            self.parse_repeated(Self::parse_composition_spec, TokenKind::Comma, TokenKind::LBrace)
        } else {
            Ok(Vec::default())
        }
    }

    /// <comp_spec> ::= <ident> | <ident> "as" <ident>
    fn parse_composition_spec(&mut self) -> ParseResult {
        let type_name = self.parse_type_spec()
            .map_err(|err| err.into_non_fatal())?;

        let (alias, loc) = if self.tokens.check_next(|tok| tok.kind == TokenKind::As) {
            self.tokens.accept(TokenKind::As)
                .map_err(|err| ParseErr::Fatal(err))?;

            let alias = self.parse_ident()?;
            let loc = SourceRange::spanned(type_name.as_ref(), alias.as_ref());
            (Some(alias), loc)
        } else {
            (None, type_name.source_range())
        };

        Ok(Ast::CompositionSpec(CompositionSpecNode {
            composed_type: type_name,
            alias,
            location: loc,
        }).into())
    }

    /// <prop> ::= <ident> ":" <type_spec>
    fn parse_prop(&mut self) -> ParseResult {
        let prop_name = self.parse_ident()
            .map_err(|err| err.into_fatal())?;

        self.tokens.accept(TokenKind::Colon)
            .map_err(|err| ParseErr::Fatal(err))?;

        let type_spec = self.parse_type_spec()?;

        let loc = SourceRange::spanned(prop_name.as_ref(), type_spec.as_ref());

        Ok(Ast::FieldDeclaration(FieldDeclarationNode {
            name: prop_name,
            tp: type_spec,
            location: loc,
        }).into())
    }
}
