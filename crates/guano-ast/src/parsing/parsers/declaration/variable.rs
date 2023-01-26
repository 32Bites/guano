use guano_syntax::{
    consts::{Keyword, Punctuation},
    node, Child, SyntaxKind,
};

use crate::parsing::{
    combinators::{alternation, tuple, Combinators},
    error::Res,
    parsers::{
        expression::expr,
        ignorable::eat_ignorable,
        symbols::{identifier::iden, ty::ty},
    },
    ParseContext, Parser,
};

pub fn var<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let mut children = var_qualifiers(context)?;

    let (kind, ws, name) = tuple((var_kind, eat_ignorable, iden.expected())).parse(context)?;
    children.push(kind);
    children.extend(ws);
    children.push(name);

    let (ty, value, ws, semi) = tuple((
        eat_ignorable.then(var_type).optional(),
        eat_ignorable.then(var_value).optional(),
        eat_ignorable,
        Punctuation::SEMICOLON.expected(),
    ))
    .parse(context)?;

    if let Some((ws, ty)) = ty {
        children.extend(ws);
        children.push(ty);
    }

    if let Some((ws, value)) = value {
        children.extend(ws);
        children.push(value);
    }

    children.extend(ws);
    children.push(semi);

    Ok(node(SyntaxKind::VAR, children))
}

pub fn var_value<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (eq, ws, expr) = tuple((Punctuation::EQ, eat_ignorable, expr)).parse(context)?;

    let mut children = vec![eq];
    children.extend(ws);
    children.push(expr);

    Ok(node(SyntaxKind::VAR_VALUE, children))
}

pub fn var_type<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (colon, ws, ty) = tuple((Punctuation::COLON, eat_ignorable, ty)).parse(context)?;

    let mut children = vec![colon];
    children.extend(ws);
    children.push(ty);

    Ok(node(SyntaxKind::VAR_TYPE, children))
}

pub fn var_kind<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let kw = alternation((Keyword::VAR, Keyword::LET)).parse(context)?;

    Ok(node(SyntaxKind::VAR_KIND, vec![kw]))
}

pub fn var_qualifiers<'source>(context: &mut ParseContext<'source>) -> Res<'source, Vec<Child>> {
    let mut children = vec![];
    if let Some((kw, ws)) = Keyword::PUB.then(eat_ignorable).optional().parse(context)? {
        children.push(kw);
        children.extend(ws);
    }

    if let Some((kw, ws)) = Keyword::STATIC
        .then(eat_ignorable)
        .optional()
        .parse(context)?
    {
        children.push(kw);
        children.extend(ws);
    }

    Ok(children)
}
