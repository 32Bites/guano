use guano_syntax::{leaf, Node, SyntaxKind};

use crate::parsing::{
    combinators::{alternation, regex},
    error::Res,
    ParseContext, Parser,
};

pub fn comment<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    alternation((line_comment, block_comment)).parse(context)
}

pub fn line_comment<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    let comment = regex(r"^//[^\n]*\n?").parse(context)?;

    Ok(leaf(SyntaxKind::COMMENT, comment))
}

pub fn block_comment<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    // let comment = block_comment_inner(context)?;
    // let comment = regex(r"^/*(?:[^*]|\*[^/])*\*/").parse(context)?;
    let comment = regex(r"^/[*](?:[^*]|(?:[*][^/]))*[*]+/").parse(context)?;

    Ok(leaf(SyntaxKind::COMMENT, comment))
}
