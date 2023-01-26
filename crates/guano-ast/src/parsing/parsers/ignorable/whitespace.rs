use guano_syntax::{leaf, Child, SyntaxKind};

use crate::parsing::{combinators::regex, error::Res, ParseContext, Parser};

pub fn whitespace<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let whitespace = regex(r"^\s+").parse(context)?;

    Ok(leaf(SyntaxKind::WHITESPACE, whitespace))
}
