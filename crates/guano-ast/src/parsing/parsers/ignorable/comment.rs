use guano_syntax::{leaf, Child, SyntaxKind};

use crate::parsing::{
    combinators::{alternation, regex},
    error::Res,
    ParseContext, Parser,
};

pub fn comment<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    alternation((line_comment, block_comment)).parse(context)
}

pub fn line_comment<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let comment = regex(r"^//[^\n]*\n?").parse(context)?;

    Ok(leaf(SyntaxKind::COMMENT, comment))
}

pub fn block_comment<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    // let comment = block_comment_inner(context)?;
    // let comment = regex(r"^/*(?:[^*]|\*[^/])*\*/").parse(context)?;
    let comment = regex(r"^/[*](?:[^*]|(?:[*][^/]))*[*]+/").parse(context)?;

    Ok(leaf(SyntaxKind::COMMENT, comment))
}
