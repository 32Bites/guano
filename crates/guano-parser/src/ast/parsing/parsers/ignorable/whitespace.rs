use guano_syntax::{leaf, Node, SyntaxKind};

use crate::ast::parsing::{combinators::regex, error::Res, ParseContext, Parser};

pub fn whitespace<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    let whitespace = regex(r"^\s+").parse(context)?;

    Ok(leaf(SyntaxKind::WHITESPACE, whitespace))
}
