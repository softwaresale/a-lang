use crate::ast::{AssignmentNode, Ast, TypeSpecNode, VariableDeclarationNode};
use crate::error::parse::ParseErr;
use crate::location::SourceRange;
use crate::parser::{Parser, ParseResult};
use crate::token::TokenKind;
use crate::types::{Type, VariableDeclarationMode};

impl<'input> Parser<'input> {
    pub(crate) fn parse_block(&mut self) -> ParseResult {
        self.tokens.accept(TokenKind::LBrace)
            .map_err(|err| ParseErr::NonFatal(err))?;

        let stmts = self.parse_repeated(Self::parse_stmt, TokenKind::Semicolon, TokenKind::RBrace)?;

        self.tokens.accept(TokenKind::RBrace)
            .map_err(|err| ParseErr::Fatal(err))?;

        Ok(Ast::Block(stmts).into())
    }

    pub(crate) fn parse_stmt(&mut self) -> ParseResult {
        self.one_of([
            Self::parse_block,
            Self::parse_assignment_stmt,
            Self::parse_expr_stmt,
        ])
    }

    fn parse_assignment_stmt(&mut self) -> ParseResult {
        let lhs = self.one_of([
            Self::parse_var_decl,
            Self::parse_expr,
        ])?;

        self.tokens.accept(TokenKind::Assign)
            .map_err(|err| ParseErr::NonFatal(err))?;

        let rhs = self.parse_expr()?;

        let loc = SourceRange::spanned(lhs.as_ref(), rhs.as_ref());

        Ok(Ast::Assignment(AssignmentNode {
            decl: lhs,
            rhs,
            location: loc,
        }).into())
    }

    fn parse_var_decl(&mut self) -> ParseResult {
        let start_loc = self.tokens.accept(TokenKind::Let)
            .map(|tok| tok.location)
            .map_err(|err| ParseErr::NonFatal(err))?;

        let var_name = self.parse_ident()
            .map_err(|err| err.into_fatal())?;

        let (type_spec, loc) = if self.tokens.check_next(|tok| tok.kind == TokenKind::Colon) {
            self.tokens.next();
            let ast = self.parse_type_spec()?;
            let loc = SourceRange::spanned(&start_loc, ast.as_ref());
            (ast, loc)
        } else {
            let loc = SourceRange::spanned(&start_loc, var_name.as_ref());
            let ast = Ast::TypeSpec(TypeSpecNode {
                tp: Type::Unknown,
                location: Default::default(),
            }).into();
            (ast, loc)
        };

        Ok(Ast::VariableDeclaration(VariableDeclarationNode {
            decl_mode: VariableDeclarationMode::Const,
            name: var_name,
            tp: type_spec,
            location: loc,
        }).into())
    }

    fn parse_expr_stmt(&mut self) -> ParseResult {
        self.parse_expr()
    }
}
