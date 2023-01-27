use guano_syntax::{
    consts::{Keyword, Punctuation},
    node, Child, SyntaxKind,
};

use crate::parsing::{
    combinators::{alternation, tuple, Combinators},
    error::Res,
    parsers::{
        ignorable::{eat_ignorable, IgnorableParser},
        symbols::{identifier::iden, path::path, ty::ty},
    },
    ParseContext, Parser,
};

pub fn class<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (pub_kw, (l_ws, class_kw, r_ws), name) = tuple((
        Keyword::PUB.optional(),
        Keyword::CLASS.padded(),
        iden.expected(),
    ))
    .parse(context)?;

    let mut children = vec![];

    if let Some(kw) = pub_kw {
        children.push(kw);
    }
    children.extend(l_ws);
    children.push(class_kw);
    children.extend(r_ws);
    children.push(name);

    let (l_ws, extends, r_ws, body) = tuple((
        eat_ignorable,
        class_extends.optional(),
        eat_ignorable,
        class_body.expected(),
    ))
    .parse(context)?;

    children.extend(l_ws);
    if let Some(extends) = extends {
        children.push(extends);
    }
    children.extend(r_ws);
    children.push(body);

    Ok(node(SyntaxKind::CLASS, children))
}

pub fn class_extends<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (col, path) = Punctuation::COLON
        .then(eat_ignorable.then(path).optional())
        .parse(context)?;
    let mut children = vec![col];

    if let Some((ws, path)) = path {
        children.extend(ws);
        children.push(path);
    }

    Ok(node(SyntaxKind::CLASS_EXTENDS, children))
}

pub fn class_body<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let body = alternation((class_block, Punctuation::SEMICOLON)).parse(context)?;
    Ok(node(SyntaxKind::CLASS_BODY, vec![body]))
}

pub fn class_block<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (l_curly, l_ws, fields, r_ws, r_curly) = tuple((
        Punctuation::LEFT_CURLY,
        eat_ignorable,
        class_field.padded().repeated(),
        eat_ignorable,
        Punctuation::RIGHT_CURLY.expected(),
    ))
    .parse(context)?;

    let mut children = vec![l_curly];
    children.extend(l_ws);

    for (l_ws, field, r_ws) in fields {
        children.extend(l_ws);
        children.push(field);
        children.extend(r_ws);
    }

    children.extend(r_ws);
    children.push(r_curly);

    Ok(node(SyntaxKind::CLASS_BLOCK, children))
}

pub fn class_field<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (pub_kw, name, col, ty, ws, semi) = tuple((
        Keyword::PUB.then(eat_ignorable).optional(),
        iden,
        Punctuation::COLON.expected().padded(),
        ty.expected(),
        eat_ignorable,
        Punctuation::SEMICOLON.expected(),
    ))
    .parse(context)?;
    let mut children = vec![];

    if let Some((kw, ws)) = pub_kw {
        children.push(kw);
        children.extend(ws);
    }
    children.push(name);

    let (l_ws, col, r_ws) = col;
    children.extend(l_ws);
    children.push(col);
    children.extend(r_ws);

    children.push(ty);
    children.extend(ws);
    children.push(semi);

    Ok(node(SyntaxKind::CLASS_FIELD, children))
}
