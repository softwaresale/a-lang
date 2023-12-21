use crate::input::SourceInput;
use crate::lexer::Lexer;
use crate::literal::Literal;
use crate::token::TokenKind;

#[test]
fn lex_eof() {
    let input = SourceInput::raw("");
    let mut lexer = Lexer::new(&input);
    let result = lexer.scan_next().expect("Lexer should not error");
    assert_eq!(result.kind, TokenKind::EOF)
}

#[test]
fn lex_str_literal() {
    let input = SourceInput::raw("\"charlie sale\"");
    let mut lexer = Lexer::new(&input);
    let result = lexer.scan_next().expect("Lexer should not error");
    assert_eq!(result.kind, TokenKind::Lit(Literal::String("charlie sale".to_string())));
}
