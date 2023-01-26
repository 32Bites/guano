use guano_syntax::{
    consts::{Keyword, Punctuation},
    node, Child, SyntaxKind,
};

use crate::parsing::{
    combinators::{alternation, tuple, Combinators},
    error::Res,
    parsers::ignorable::eat_ignorable,
    ParseContext, Parser,
};

use super::identifier::iden;

pub fn name<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let name = alternation((Keyword::THIS, iden)).parse(context)?;

    Ok(node(SyntaxKind::NAME, vec![name]))
}

pub fn path_segment<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (col, ws, name) = tuple((Punctuation::COLON2, eat_ignorable, name)).parse(context)?;
    let mut children = vec![col];
    children.extend(ws);
    children.push(name);

    Ok(node(SyntaxKind::PATH_SEGMENT, children))
}

pub fn path<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (mut first_segment, other_segments) =
        tuple((name, eat_ignorable.then(path_segment).repeated())).parse(context)?;
    first_segment = node(SyntaxKind::PATH_SEGMENT, vec![first_segment]);

    let mut children = vec![first_segment];
    for (ws, seg) in other_segments {
        children.extend(ws);
        children.push(seg);
    }

    Ok(node(SyntaxKind::PATH, children))
}
