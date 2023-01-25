use guano_syntax::{
    consts::{Keyword, Punctuation},
    node, Node, SyntaxKind,
};

use crate::ast::parsing::{
    combinators::{alternation, tuple, Combinators},
    error::Res,
    parsers::ignorable::eat_ignorable,
    ParseContext, Parser,
};

use super::iden::iden;

pub fn name<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    let name = alternation((Keyword::THIS, iden)).parse(context)?;

    Ok(node(SyntaxKind::NAME, vec![name]))
}

pub fn path_segment<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    let (col, r_ws, name) = tuple((Punctuation::COLON2, eat_ignorable, name)).parse(context)?;
    let mut children = vec![col];
    children.extend(r_ws);
    children.push(name);

    Ok(node(SyntaxKind::PATH_SEGMENT, children))
}

pub fn path<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    let (first_segment, other_segments) =
        tuple((name, eat_ignorable.then(path_segment).repeated())).parse(context)?;
    let mut children = vec![node(SyntaxKind::PATH_SEGMENT, vec![first_segment])];

    for (whitespace, segment) in other_segments {
        children.extend(whitespace);
        children.push(segment);
    }

    Ok(node(SyntaxKind::PATH, children))
}
