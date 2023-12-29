use crate::ast::{Ast, TypeSpecNode};
use crate::error::parse::ParseErr;
use crate::location::HasLocation;
use crate::parser::{Parser, ParseResult};
use crate::token::TokenKind;
use crate::types::Type;

impl<'input> Parser<'input> {

    pub(crate) fn parse_type_spec(&mut self) -> ParseResult {
        let spec = self.parse_type()?;
        Ok(Ast::TypeSpec(spec).into())
    }

    fn parse_type(&mut self) -> Result<TypeSpecNode, ParseErr> {
        self.parse_scalar_type()
    }

    fn parse_function_type(&mut self) -> Result<Type, ParseErr> {
        todo!()
    }

    /// <scalar_type> ::= TYPE_KEYWORD
    fn parse_scalar_type(&mut self) -> Result<TypeSpecNode, ParseErr> {
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
