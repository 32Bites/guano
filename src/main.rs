use clap::Parser;

mod cli;

#[cfg(feature = "lexer")]
mod lexer;

fn main() {
    let cli = cli::CommandLine::parse();
    println!("{:?}", cli);

    cli.run();
}
