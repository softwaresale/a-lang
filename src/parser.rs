mod expr;
mod funs;

use std::collections::HashMap;
use crate::ast::{Ast};
use crate::error::source::SourceError;
use crate::lexer::token_stream::TokenStream;
use crate::token::TokenKind;

pub type ParseResult = Result<Box<Ast>, Vec<SourceError>>;

pub struct Parser<'input> {
    /// the stream of input tokens available to us
    tokens: TokenStream<'input>,
}

// Basic utility functions for parser

impl<'input> Parser<'input> {
    pub fn new(tokens: TokenStream<'input>) -> Self {
        Self {
            tokens
        }
    }

    // parse rules
    pub fn parse_compilation_unit(&mut self) -> ParseResult {
        self.parse_atom()
    }

    // parsing utilities
    pub(crate) fn one_of<const COUNT: usize>(&mut self, actions: [fn(&mut Self) -> ParseResult; COUNT]) -> ParseResult {

        let mut error_map = HashMap::<usize, Vec<SourceError>>::new();

        for action in actions {
            let starting = self.tokens.save();
            let result = action(self);
            match result {
                Ok(matched) => {
                    return Ok(matched)
                }
                Err(err) => {
                    // figure out the longest
                    let distance = self.tokens.cursor() - starting;
                    error_map.insert(distance, err);
                    self.tokens.restore();
                }
            }
        }

        let max_key = *error_map.keys().max().unwrap();
        let errors = error_map.remove(&max_key).unwrap();
        Err(errors)
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
    ) -> Result<Vec<Box<Ast>>, Vec<SourceError>> {
        let mut items = Vec::<Box<Ast>>::new();
        loop {
            // if next token is the stop token, we are done
            if self.tokens.check_next(|next_tok| next_tok.kind == stop_token) {
                break;
            }

            // otherwise, parse the item
            let item = rule(self)?;
            items.push(item);

            // find the delimiter
            let next_token = self.tokens.next().unwrap();
            if next_token.kind == stop_token {
                self.tokens.putback();
                break;
            } else if next_token.kind != delimiter {
                // we have an error
                let err = SourceError::new(format!("Expected '{:?}' or '{:?}', but got '{:?}' instead", delimiter, stop_token, next_token.kind), next_token.location);
                return Err(vec![err]);
            }
        }

        Ok(items)
    }
}
