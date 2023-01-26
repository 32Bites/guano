use guano_syntax::{
    consts::{Keyword, Punctuation},
    node, Node, SyntaxKind,
};

use crate::parsing::{
    combinators::{alternation, tuple, Combinators},
    error::Res,
    parsers::{
        expression::expr,
        ignorable::eat_ignorable,
        symbols::{iden::iden, ty::ty},
    },
    ParseContext, Parser,
};

pub fn var<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    let mut children = vec![];
    let (qualifiers, kind, ws, name) = tuple((
        var_qualifiers.then(eat_ignorable).optional(),
        var_kind,
        eat_ignorable,
        iden.expected(),
    ))
    .parse(context)?;

    if let Some((qualifiers, ws)) = qualifiers {
        children.push(qualifiers);
        children.extend(ws);
    }

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

pub fn var_value<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    let (eq, ws, expr) = tuple((Punctuation::EQ, eat_ignorable, expr)).parse(context)?;

    let mut children = vec![eq];
    children.extend(ws);
    children.push(expr);

    Ok(node(SyntaxKind::VAR_VALUE, children))
}

pub fn var_type<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    let (colon, ws, ty) = tuple((Punctuation::COLON, eat_ignorable, ty)).parse(context)?;

    let mut children = vec![colon];
    children.extend(ws);
    children.push(ty);

    Ok(node(SyntaxKind::VAR_TYPE, children))
}

pub fn var_kind<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    let kw = alternation((Keyword::VAR, Keyword::LET)).parse(context)?;

    Ok(node(SyntaxKind::VAR_KIND, vec![kw]))
}

pub fn var_qualifiers<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    let has_pub = Keyword::PUB
        .then(eat_ignorable.then(Keyword::STATIC).optional())
        .map(|(first, rest)| {
            let mut children = vec![first];

            if let Some((ws, other)) = rest {
                children.extend(ws);
                children.push(other);
            }

            children
        });
    let only_static = Keyword::STATIC.map(|t| vec![t]);

    let children = only_static.or(has_pub).parse(context)?;
    let qualifiers = node(SyntaxKind::VAR_QUALIFIERS, children);

    Ok(qualifiers)
}
