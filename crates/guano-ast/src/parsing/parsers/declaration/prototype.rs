use guano_syntax::{
    consts::{Keyword, Punctuation},
    node, Child, SyntaxKind,
};

use crate::parsing::{
    combinators::{tuple, Combinators},
    error::Res,
    parsers::{
        ignorable::eat_ignorable,
        symbols::{identifier::iden, path::path},
    },
    ParseContext, Parser,
};

use super::function::funcs;

pub fn proto<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (pub_kw, proto_kw, l_ws, name, r_ws) = tuple((
        Keyword::PUB.then(eat_ignorable).optional(),
        Keyword::PROTO,
        eat_ignorable,
        iden.expected(),
        eat_ignorable,
    ))
    .parse(context)?;

    let mut children = vec![];

    if let Some((kw, ws)) = pub_kw {
        children.push(kw);
        children.extend(ws);
    }
    children.push(proto_kw);
    children.extend(l_ws);
    children.push(name);
    children.extend(r_ws);

    let (extends, body) = tuple((
        proto_extends.then(eat_ignorable).optional(),
        proto_body.expected(),
    ))
    .parse(context)?;

    if let Some((extends, ws)) = extends {
        children.push(extends);
        children.extend(ws);
    }
    children.push(body);

    Ok(node(SyntaxKind::PROTO, children))
}

pub fn proto_extends<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let extensions = tuple((
        path.map(|p| node(SyntaxKind::PROTO_EXTENSION, vec![p])),
        eat_ignorable.then(proto_extension).repeated(),
    ));

    let (col, extensions) = Punctuation::COLON
        .then(eat_ignorable.then(extensions).optional())
        .parse(context)?;

    let mut children = vec![col];

    if let Some((ws, (first_extension, other_extensions))) = extensions {
        children.extend(ws);
        children.push(first_extension);

        for (ws, extension) in other_extensions {
            children.extend(ws);
            children.push(extension);
        }
    }

    Ok(node(SyntaxKind::PROTO_EXTENDS, children))
}

pub fn proto_extension<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (plus, ws, path) =
        tuple((Punctuation::PLUS, eat_ignorable, path.expected())).parse(context)?;

    let mut children = vec![plus];
    children.extend(ws);
    children.push(path);

    Ok(node(SyntaxKind::PROTO_EXTENSION, children))
}

#[inline]
pub fn proto_body<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    funcs(SyntaxKind::PROTO_BODY)(context)
}
