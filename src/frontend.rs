use crate::error::source::SourceError;
use crate::frontend::ast::Ast;
use crate::frontend::input::SourceInput;
use crate::frontend::lexer::Lexer;
use crate::frontend::parser::Parser;

pub mod ast;
pub mod input;
mod lexer;
pub mod location;
mod token;
mod parser;

pub fn parse_input_source(input: &SourceInput) -> Result<Box<Ast>, Vec<SourceError>> {
    // eagerly read into tokens
    let token_stream = Lexer::new(input)
        .into_token_stream();
    if let Err(lexing_error) = token_stream {
        return Err(vec![lexing_error])
    }

    let token_stream = token_stream.unwrap();

    println!("--Tokens--");
    for token in token_stream.tokens() {
        println!("{:?}", token)
    }
    println!("--End Tokens--");

    let parser = Parser::new(token_stream);
    parser.parse_compilation_unit()
}
