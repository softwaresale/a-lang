use crate::ast::{ArrayAccessNode, Ast, BinaryOpNode, CondExprNode, IdentNode, LitNode, UnaryOpNode};
use crate::error::parse::ParseErr;
use crate::error::source::SourceError;
use crate::literal::{Literal};
use crate::location::SourceRange;
use crate::operators::{BinaryOp, UnaryOp};
use crate::parser::{Parser, ParseResult};
use crate::token::TokenKind;

/// The expression parser

impl<'input> Parser<'input> {
    /// <expr> ::= TODO
    pub(crate) fn parse_expr(&mut self) -> ParseResult {
        self.one_of([
            Self::parse_conditional_expr,
            Self::parse_binary_expr,
        ])
    }

    fn parse_conditional_expr(&mut self) -> ParseResult {
        let if_tok_location = self.tokens.accept(TokenKind::If)
            .map(|tok| tok.location)
            .map_err(|err| ParseErr::NonFatal(err))?;

        self.tokens.accept(TokenKind::LParen)
            .map_err(|err| ParseErr::Fatal(err))?;

        let conditional = self.parse_expr()?;

        self.tokens.accept(TokenKind::RParen)
            .map_err(|err| ParseErr::Fatal(err))?;

        let true_block = self.parse_block()?;

        let (false_block, loc) = if self.tokens.check_next(|token| match token.kind {
            TokenKind::Else => true,
            _ => false
        }) {
            // take an else branch
            self.tokens.next();

            let ast = self.parse_block()?;
            let loc = SourceRange::spanned(&if_tok_location, ast.as_ref());
            (ast, loc)
        } else {
            let ast = Box::new(Ast::Block(vec![Ast::Literal(LitNode {
                lit: Literal::Unit,
                location: Default::default(),
            }).into()]));

            let loc = SourceRange::spanned(&if_tok_location, true_block.as_ref());
            (ast, loc)
        };

        Ok(Ast::CondExpr(CondExprNode {
            cond: conditional,
            true_branch: true_block,
            false_branch: false_block,
            location: loc,
        }).into())
    }

    fn parse_binary_expr(&mut self) -> ParseResult {
        let lhs = self.parse_unary_op()?;
        self.parse_binary_expr_rec(lhs, 0)
    }

    fn parse_binary_expr_rec(&mut self, mut lhs: Box<Ast>, min_prec: u8) -> ParseResult {
        let result_ast = loop {
            let lookahead = self.tokens.peek()
                .cloned()
                .and_then(|token| BinaryOp::try_from(token).ok());

            if !lookahead.as_ref().is_some_and(|op| op.precedence() >= min_prec) {
                break lhs;
            }

            let op = lookahead.unwrap();
            self.tokens.advance(1);

            let mut rhs = self.parse_unary_op()?;

            let mut inner_op = self.tokens.peek()
                .cloned()
                .and_then(|token| BinaryOp::try_from(token).ok());

            while inner_op.as_ref().is_some_and(|inner_op| inner_op.precedence() > op.precedence()) {
                let prec = op.precedence() + if inner_op.unwrap().precedence() > op.precedence() { 1 } else { 0 };
                rhs = self.parse_binary_expr_rec(rhs, prec)?;
                inner_op = self.tokens.peek()
                    .cloned()
                    .and_then(|token| BinaryOp::try_from(token).ok());
            }

            let loc = SourceRange::spanned(lhs.as_ref(), rhs.as_ref());

            lhs = Box::new(Ast::BinaryOp(BinaryOpNode {
                op,
                lhs,
                rhs,
                location: loc,
            }));
        };

        Ok(result_ast)
    }

    /// <unary_op> ::= <member_access> | UNARY_OP <unary_op>
    fn parse_unary_op(&mut self) -> ParseResult {

        let next_token = self.tokens.next().unwrap();
        let unary_operator = match next_token.kind {
            TokenKind::Minus => Some(UnaryOp::Neg),
            TokenKind::Times => Some(UnaryOp::Deref),
            TokenKind::Ref => Some(UnaryOp::Ref),
            _ => None
        };
        if let Some(op) = unary_operator {
            let start_loc = next_token.location;
            let child = self.parse_unary_op()?;
            let loc = SourceRange::spanned(&start_loc, child.as_ref());
            Ok(Ast::UnaryOp(UnaryOpNode {
                op,
                child,
                location: loc,
            }).into())
        } else {
            self.tokens.putback();
            self.parse_member_access()
        }
    }

    /// <member_access> ::= <atom> | <member_access> '.' ( <fun_call> | <ident> )
    fn parse_member_access(&mut self) -> ParseResult {
        self.parse_infix_op(
            [BinaryOp::Access, BinaryOp::ChainedAccess],
            Self::parse_atom,
            Self::parse_member_access,
        )
    }

    /// <atom> ::= <literal> | <ident> | <parens_expr> | <fun_call> | <array_access>
    fn parse_atom(&mut self) -> ParseResult {
        let result = self.one_of([
            Self::parse_parens_expr,
            Self::parse_fun_call,
            Self::parse_literal,
            Self::parse_ident
        ]);

        match result {
            Ok(ast) => {
                Ok(ast)
            }
            Err(err) => {
                self.errors.push(err.clone().into());
                Err(err)
            }
        }
    }

    /// <array_access> ::= <expr> '\[' <expr> '\]'
    fn parse_array_access(&mut self) -> ParseResult {
        let derefed = self.parse_expr()
            .map_err(|err| err.into_non_fatal())?;

        self.tokens.accept(TokenKind::LBracket)
            .map_err(|err| ParseErr::NonFatal(err))?;

        let access = self.parse_expr()
            .map_err(|err| err.into_fatal())?;

        let end_tok = self.tokens.accept(TokenKind::RBracket)
            .map_err(|err| ParseErr::Fatal(err))?;

        let loc = SourceRange::spanned(derefed.as_ref(), end_tok);

        Ok(Ast::ArrayAccess(ArrayAccessNode {
            derefed,
            access,
            location: loc,
        }).into())
    }

    /// <parens_result> ::= '(' <expr> ')'
    fn parse_parens_expr(&mut self) -> ParseResult {
        // try to accept an lparen. If doesn't match, then non-fatal cause it could be something else
        self.tokens.accept(TokenKind::LParen)
            .map_err(|err| ParseErr::NonFatal(err))?;
        let expr = self.parse_expr()?;
        // however, if we started a parenthesized expr, then we need to finish it, so this one would
        // be fatal
        self.tokens.accept(TokenKind::RParen)
            .map_err(|err| ParseErr::Fatal(err))?;

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
                    let err = SourceError::new(format!("Expected identifier but got '{:?}' instead", other), token.location);
                    Err(ParseErr::NonFatal(err))
                }
            }
        })
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
                    let err = SourceError::new(format!("Expected identifier but got '{:?}' instead", other), token.location);
                    Err(ParseErr::NonFatal(err))
                }
            }
        })
    }
}

/// binary operator infix power stuff
impl<'input> Parser<'input> {
    pub fn parse_infix_op<const COUNT: usize>(
        &mut self,
        accepted_ops: [BinaryOp; COUNT],
        lhs_rule: fn(&mut Self) -> ParseResult,
        rhs_rule: fn(&mut Self) -> ParseResult,
    ) -> ParseResult {
        let lhs = lhs_rule(self)?;
        // try to get an infix token
        let Some(infix_tok) = self.tokens.peek() else { return Ok(lhs) };
        let Ok(op) = BinaryOp::try_from(infix_tok.clone()) else { return Ok(lhs) };

        if !accepted_ops.contains(&op) {
            return Ok(lhs);
        }

        // we kept the infix token, advance by one
        self.tokens.advance(1);

        // parse the right hand
        let rhs = rhs_rule(self)?;

        let loc = SourceRange::spanned(lhs.as_ref(), rhs.as_ref());

        Ok(Ast::BinaryOp(BinaryOpNode {
            op,
            lhs,
            rhs,
            location: loc,
        }).into())
    }
}
