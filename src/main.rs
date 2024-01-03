mod args;
mod error;
mod literal;
mod operators;
mod types;
mod frontend;
mod analysis;
mod symtab;

use std::error::Error;
use std::process::{ExitCode, ExitStatus};
use clap::Parser as ClapParser;
use crate::args::ProgramArgs;
use crate::frontend::input::SourceInput;
use crate::frontend::parse_input_source;

fn main() -> Result<ExitCode, Box<dyn Error>> {

    let args = ProgramArgs::parse();

    if args.input_files.is_empty() {
        println!("No input files. Nothing to do");
        return Ok(ExitCode::SUCCESS)
    }

    let source_input  = SourceInput::open(args.input_files.first().unwrap())?;
    if source_input.contains_non_ascii() {
        eprintln!("Error: input cannot contain non-ascii characters");
        return Ok(ExitCode::FAILURE);
    }

    let ast = parse_input_source(&source_input);

    let Ok(ast) = ast else {
        let errors = ast.unwrap_err();
        eprintln!("Parsing error occurred");
        for error in errors {
            let report = source_input.create_error_report(error);
            eprintln!("{}", report);
        }

        return Ok(ExitCode::FAILURE)
    };

    println!("--AST--");
    println!("{:#?}", ast);

    Ok(ExitCode::SUCCESS)
}
