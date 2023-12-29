mod expr;
mod funs;
mod stmt;
mod tp;

use std::collections::HashMap;
use crate::ast::{Ast};
use crate::error::parse::ParseErr;
use crate::error::source::SourceError;
use crate::lexer::token_stream::TokenStream;
use crate::token::TokenKind;

pub type ParseResult = Result<Box<Ast>, ParseErr>;

pub struct Parser<'input> {
    /// the stream of input tokens available to us
    tokens: TokenStream<'input>,
    /// source errors we have encountered along the way
    errors: Vec<SourceError>,
}

// Basic utility functions for parser

impl<'input> Parser<'input> {
    pub fn new(tokens: TokenStream<'input>) -> Self {
        Self {
            tokens,
            errors: Vec::default()
        }
    }

    // parse rules
    pub fn parse_compilation_unit(mut self) -> Result<Box<Ast>, Vec<SourceError>> {
        let parse_result = self.parse_expr();
        match parse_result {
            Ok(ast) => Ok(ast),
            Err(_) => Err(self.errors)
        }
    }

    // parsing utilities
    pub(crate) fn one_of<const COUNT: usize>(&mut self, actions: [fn(&mut Self) -> ParseResult; COUNT]) -> ParseResult {
        let mut distances = [0usize; COUNT];
        let mut errors = HashMap::<usize, SourceError>::with_capacity(COUNT);

        for (idx, action) in actions.iter().enumerate() {
            let starting = self.tokens.save();
            let result = action(self);
            match result {
                Ok(matched) => {
                    return Ok(matched)
                }
                Err(err) => {
                    match err {
                        ParseErr::Fatal(fatal_error) => {
                            return Err(ParseErr::Fatal(fatal_error))
                        }
                        ParseErr::NonFatal(non_fatal_error) => {
                            let distance = self.tokens.cursor() - starting;
                            distances[idx] = distance;
                            errors.insert(idx, non_fatal_error);
                            self.tokens.restore();
                        }
                    }
                }
            }
        }

        let error_idx = distances.into_iter()
            .enumerate()
            .max_by_key(|(_, dist)| *dist)
            .map(|(idx, _)| idx)
            .unwrap();
        let err = errors.remove(&error_idx).unwrap();
        // self.errors.push(err.clone());
        Err(ParseErr::Fatal(err))
    }

    /// parse a repeated rule
    /// rule: the rule to parse each time
    /// delimiter: the token to break between the two
    /// stop_token: when this token is encountered, stop parsing
    pub(crate) fn parse_repeated(
        &mut self,
        rule: fn(&mut Self) -> ParseResult,
        delimiter: TokenKind,
        stop_token: TokenKind
    ) -> Result<Vec<Box<Ast>>, ParseErr> {
        let mut items = Vec::<Box<Ast>>::new();
        loop {
            // if next token is the stop token, we are done
            if self.tokens.check_next(|next_tok| next_tok.kind == stop_token) {
                break;
            }

            // otherwise, parse the item
            let item = rule(self);
            match item {
                Ok(item) => {
                    items.push(item);
                }
                Err(parse_err) => {
                    match parse_err {
                        ParseErr::Fatal(fatal_err) => {
                            self.errors.push(fatal_err);
                        }
                        ParseErr::NonFatal(non_fatal_error) => {
                            self.errors.push(non_fatal_error);
                        }
                    }

                    // error recovery: skip to either the next delim or stop token
                    self.tokens.skip_to(|tok| tok.kind == delimiter || tok.kind == stop_token)
                }
            }

            // find the delimiter
            let next_token = self.tokens.next().unwrap();
            if next_token.kind == stop_token {
                self.tokens.putback();
                break;
            } else if next_token.kind != delimiter {
                // we have an error
                let err = SourceError::new(format!("Expected '{:?}' or '{:?}', but got '{:?}' instead", delimiter, stop_token, next_token.kind), next_token.location);
                return Err(ParseErr::Fatal(err));
            }
        }

        Ok(items)
    }
}
