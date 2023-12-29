use crate::ast::{Ast, TypeSpecNode};
use crate::error::parse::ParseErr;
use crate::location::{HasLocation, SourceRange};
use crate::parser::{Parser, ParseResult};
use crate::token::TokenKind;
use crate::types::Type;

impl<'input> Parser<'input> {

    pub(crate) fn parse_type_spec(&mut self) -> ParseResult {
        let spec = self.parse_type()?;
        Ok(Ast::TypeSpec(spec).into())
    }

    fn parse_type(&mut self) -> Result<TypeSpecNode, ParseErr> {
        self.parse_non_scalar_type()
    }

    fn parse_function_type(&mut self) -> Result<Type, ParseErr> {
        todo!()
    }

    /// <non_scalar_type> ::= "\[" "\]" <non_scalar_type> | "&" <non_scalar_type> | <optional_type>
    fn parse_non_scalar_type(&mut self) -> Result<TypeSpecNode, ParseErr> {
        if self.tokens.check_next(|tok| match tok.kind {
            TokenKind::LBracket |
            TokenKind::Ref => true,
            _ => false,
        }) {

            let next_tok = self.tokens.next().unwrap();
            let kind = next_tok.kind;
            let loc = next_tok.location;
            let (higher_level_type, location) = match kind {
                TokenKind::LBracket => {
                    self.tokens.accept(TokenKind::RBracket)
                        .map_err(|err| ParseErr::Fatal(err))?;
                    let inner_type = self.parse_non_scalar_type()?;
                    let view_type = Type::View(inner_type.tp.into());
                    let loc = SourceRange::spanned(&loc, &inner_type.location);
                    (view_type, loc)
                }
                TokenKind::Ref => {
                    let inner_type = self.parse_non_scalar_type()?;
                    let loc = SourceRange::spanned(&loc, &inner_type.location);
                    (Type::Reference(inner_type.tp.into()), loc)
                }
                _ => unreachable!()
            };

            Ok(TypeSpecNode {
                tp: higher_level_type,
                location,
            })
        } else {
            self.parse_optional_type()
        }
    }

    /// <optional_type> ::= <scalar_type> "?"
    fn parse_optional_type(&mut self) -> Result<TypeSpecNode, ParseErr> {
        let inner_type = self.parse_scalar_type()?;
        if self.tokens.check_next(|tok| tok.kind == TokenKind::Nullable) {
            let end_loc = self.tokens.accept(TokenKind::Nullable)
                .map(|tok| tok.location)
                .map_err(|err| ParseErr::Fatal(err))?;

            let loc = SourceRange::spanned(&inner_type.location, &end_loc);

            let opt_type = Type::Optional(inner_type.tp.into());
            Ok(TypeSpecNode {
                tp: opt_type,
                location: loc,
            })
        } else {
            Ok(inner_type)
        }
    }

    /// <scalar_type> ::= TYPE_KEYWORD | "(" <type> ")"
    fn parse_scalar_type(&mut self) -> Result<TypeSpecNode, ParseErr> {
        if self.tokens.check_next(|tok| tok.kind == TokenKind::LParen) {
            self.parse_parened_type()
        } else {
            self.parse_type_keyword()
        }
    }

    fn parse_parened_type(&mut self) -> Result<TypeSpecNode, ParseErr> {
        let start_loc = self.tokens.accept(TokenKind::LParen)
            .map(|tok| tok.location)
            .map_err(|err| ParseErr::Fatal(err))?;

        let contained_type_spec = self.parse_type()?;
        let end_loc = self.tokens.accept(TokenKind::RParen)
            .map(|tok| tok.location)
            .map_err(|err| ParseErr::Fatal(err))?;

        let loc = SourceRange::spanned(&start_loc, &end_loc);

        Ok(TypeSpecNode {
            tp: contained_type_spec.tp,
            location: loc,
        })
    }

    fn parse_type_keyword(&mut self) -> Result<TypeSpecNode, ParseErr> {
        let type_name = self.parse_ident()
            .map_err(|err| err.into_fatal())?;
        let loc = type_name.source_range();

        let Ast::Identifier(ident_node) = *type_name else {
            panic!("Parsed something that wasn't an identifier")
        };

        let tp = Type::from(ident_node.ident.as_str());
        let tp = match tp {
            Type::Unknown => Type::UserDefined(ident_node.ident),
            tp => tp
        };

        Ok(TypeSpecNode {
            tp,
            location: loc,
        })
    }
}
