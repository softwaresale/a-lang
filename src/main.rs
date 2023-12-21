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

use std::error::Error;
use std::process::ExitCode;
use clap::Parser;
use crate::args::ProgramArgs;
use crate::input::SourceInput;
use crate::lexer::Lexer;

fn main() -> Result<ExitCode, Box<dyn Error>> {

    let args = ProgramArgs::parse();

    /*
    if args.input_files.is_empty() {
        println!("No input files. Nothing to do");
        return Ok(ExitCode::SUCCESS)
    }
     */

    let source_input  = SourceInput::raw("let const mut age = -age  something else");
    if source_input.contains_non_ascii() {
        eprintln!("Error: input cannot contain non-ascii characters");
        return Ok(ExitCode::FAILURE);
    }

    // eagerly read into tokens
    let token_stream = Lexer::new(&source_input)
        .into_token_stream()?;

    for token in token_stream.tokens() {
        println!("{:?}", token)
    }

    Ok(ExitCode::SUCCESS)
}
