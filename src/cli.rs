use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[cfg(feature = "lexer")]
use crate::lexer;

#[derive(Parser, Debug)]
// #[command(author, version, about)]
pub struct CommandLine {
    #[command(subcommand)]
    pub command: Command,
}

impl CommandLine {
    pub fn run(&self) {
        self.command.run()
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[cfg(feature = "lexer")]
    Lex {
        #[arg(default_value = ".")]
        source_files: Vec<PathBuf>,
    },
}

impl Command {
    pub fn run(&self) {
        match self {
            #[cfg(feature = "lexer")]
            Command::Lex { source_files } => {
                if source_files
                    .get(0)
                    .map_or(false, |s| s.to_str().map_or(false, |s| s == "."))
                {
                    lexer::lex_files(None)
                } else {
                    lexer::lex_files(Some(source_files))
                }
            }
        }
    }
}
