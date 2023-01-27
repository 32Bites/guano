use guano_syntax::{consts::Punctuation, node, Child, SyntaxKind};

use crate::parsing::{
    combinators::{alternation, tuple, Combinators},
    error::Res,
    parsers::ignorable::{eat_ignorable, IgnorableParser},
    ParseContext, Parser,
};

use super::path::path;

pub fn ty<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    nilable_type(context)
}

pub fn nilable_type<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let mut lhs = primary_type(context)?;

    for (ws, ques) in eat_ignorable
        .then(Punctuation::QUES)
        .repeated()
        .parse(context)?
    {
        take_mut::take(&mut lhs, |lhs| {
            let mut children = vec![lhs];
            children.extend(ws);
            children.push(ques);

            let nilable = node(SyntaxKind::NILABLE_TYPE, children);

            nilable // node(SyntaxKind::TYPE, vec![nilable])
        });
    }

    Ok(lhs)
}

pub fn primary_type<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    alternation((list_type, path)).parse(context)
}

pub fn list_type<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (l_brack, (l_ws, ty, r_ws), r_brack) = tuple((
        Punctuation::LEFT_BRACK,
        ty.expected().padded(),
        Punctuation::RIGHT_BRACK.expected(),
    ))
    .parse(context)?;
    let mut children = vec![l_brack];
    children.extend(l_ws);
    children.push(ty);
    children.extend(r_ws);
    children.push(r_brack);

    let list = node(SyntaxKind::LIST_TYPE, children);

    Ok(list)
}
