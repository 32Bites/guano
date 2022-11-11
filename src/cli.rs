use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[cfg(feature = "ast")]
use crate::ast;

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
    #[cfg(feature = "ast")]
    Ast {
        #[arg(default_value = ".")]
        source_files: Vec<PathBuf>,
    },
}

impl Command {
    pub fn run(&self) {
        match self {
            #[cfg(feature = "ast")]
            Command::Ast { source_files } => {
                if source_files
                    .get(0)
                    .map_or(false, |s| s.to_str().map_or(false, |s| s == "."))
                {
                    ast::parse(None)
                } else {
                    ast::parse(Some(source_files))
                }
            }
        }
    }
}
