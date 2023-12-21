use crate::token::Token;

pub struct TokenStream {
    /// the actual tokens
    tokens: Vec<Token>,
    /// where in the token stream we are looking
    cursor: usize,
    /// internal stack for saving cursor locations when backtracking
    cursor_stack: Vec<usize>,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            cursor: 0,
            cursor_stack: Vec::new()
        }
    }

    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }
}
