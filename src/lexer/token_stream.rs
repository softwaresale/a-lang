use crate::error::parse::ParseErr;
use crate::error::source::SourceError;
use crate::location::SourceRange;
use crate::token::{Token, TokenKind};

pub struct TokenStream<'input> {
    /// the actual tokens
    tokens: Vec<Token<'input>>,
    /// where in the token stream we are looking
    cursor: usize,
    /// internal stack for saving cursor locations when backtracking
    cursor_stack: Vec<usize>,
}

impl<'input> TokenStream<'input> {
    pub fn new(tokens: Vec<Token<'input>>) -> Self {
        Self {
            tokens,
            cursor: 0,
            cursor_stack: Vec::new()
        }
    }

    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn putback(&mut self) {
        self.putback_n(1)
    }

    pub fn putback_n(&mut self, n: usize) {
        self.cursor -= n;
    }

    /// save the current cursor onto the stack
    pub fn save(&mut self) -> usize {
        self.cursor_stack.push(self.cursor);
        self.cursor
    }

    /// restore a saved cursor into the stored cursor
    pub fn restore(&mut self) {
        if let Some(saved_cursor) = self.cursor_stack.pop() {
            self.cursor = saved_cursor
        }
    }

    pub fn advance(&mut self, amount: usize) {
        self.cursor += amount
    }

    pub fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.cursor);
        self.cursor += 1;
        token
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }

    pub fn check_next<PredT: Fn(&Token) -> bool>(&self, pred: PredT) -> bool {
        if let Some(next) = self.tokens.get(self.cursor) {
            pred(next)
        } else {
            false
        }
    }

    pub fn accept(&mut self, expected: TokenKind) -> Result<&Token, SourceError> {
        if let Some(tok) = self.next() {
            if tok.kind == expected {
                Ok(tok)
            } else {
                Err(SourceError::new(format!("Expected '{:?}' but got {:?} instead", expected, tok.kind), tok.location))
            }
        } else {
            Err(SourceError::new(format!("Expected '{:?}' but got end of token stream instead", expected), SourceRange::default()))
        }
    }

    pub fn accept_if_map<R, PredT: Fn(&Token) -> Result<R, ParseErr>>(&mut self, pred: PredT) -> Result<R, ParseErr> {
        if let Some(next) = self.next() {
            pred(next)
        } else {
            let err = SourceError::new(format!("Unexpected end of token stream instead"), SourceRange::default());
            Err(ParseErr::Fatal(err))
        }
    }

    /// skip tokens until the given predicate evaluates to true
    pub fn skip_to<PredT: Fn(&Token) -> bool>(&mut self, pred: PredT) {
        while let Some(next) = self.next() {
            if pred(next) {
                break;
            }
        }
    }
}
