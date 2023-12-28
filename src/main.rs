mod args;
mod ast;
mod error;
mod input;
mod lexer;
mod literal;
mod location;
mod operators;
mod types;
mod token;
mod parser;

use std::error::Error;
use std::process::ExitCode;
use clap::Parser as ClapParser;
use crate::args::ProgramArgs;
use crate::input::SourceInput;
use crate::lexer::Lexer;
use crate::parser::{Parser};

fn main() -> Result<ExitCode, Box<dyn Error>> {

    let args = ProgramArgs::parse();

    /*
    if args.input_files.is_empty() {
        println!("No input files. Nothing to do");
        return Ok(ExitCode::SUCCESS)
    }
     */

    let source_input  = SourceInput::raw("foo(age 12)");
    if source_input.contains_non_ascii() {
        eprintln!("Error: input cannot contain non-ascii characters");
        return Ok(ExitCode::FAILURE);
    }

    // eagerly read into tokens
    let token_stream = Lexer::new(&source_input)
        .into_token_stream()?;

    println!("--Tokens--");
    for token in token_stream.tokens() {
        println!("{:?}", token)
    }
    println!("--End Tokens--");

    let mut parser = Parser::new(token_stream);
    let ast = parser.parse_compilation_unit();
    match ast {
        Ok(ast) => {
            println!("--AST--");
            println!("{:?}", ast);
        }
        Err(errors) => {
            for error in errors {
                eprintln!("Error: {}", error)
            }
        }
    }

    Ok(ExitCode::SUCCESS)
}
