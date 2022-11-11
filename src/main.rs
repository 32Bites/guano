use clap::Parser;

mod cli;
mod source_files;

#[cfg(feature = "ast")]
mod ast;

fn main() {
    let cli = cli::CommandLine::parse();
    cli.run();
}
