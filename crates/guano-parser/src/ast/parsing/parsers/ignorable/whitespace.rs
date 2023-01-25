use guano_syntax::{leaf, SyntaxKind, Node};

use crate::ast::parsing::{ParseContext, error::Res, combinators::regex, Parser};

pub fn whitespace<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    let whitespace = regex(r"^\s+").parse(context)?;

    Ok(leaf(SyntaxKind::WHITESPACE, whitespace))
}