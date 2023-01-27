/// Guano parsing structures.
pub mod parsing;

use guano_common::rowan::ast::AstNode;
/// Syntax structures
pub use guano_syntax;
use guano_syntax::nodes::SourceFile;
use parsing::{combinators::Combinators, parsers::source_file, ParseContext, Parser};

pub fn parse_file<'source>(
    source: &'source str,
) -> (
    ParseContext<'source>,
    Result<SourceFile, parsing::error::Error<'source>>,
) {
    let mut context = ParseContext::new(source);
    let result = source_file
        .ast()
        .map(|n| SourceFile::cast(n).unwrap())
        .parse(&mut context);

    (context, result)
}
