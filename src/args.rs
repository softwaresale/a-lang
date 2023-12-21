use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about)]
pub struct ProgramArgs {
    /// the input files to compile
    pub input_files: Vec<PathBuf>
}
