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

pub fn import<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (kw, l_ws, path, r_ws) = tuple((
        Keyword::IMPORT,
        eat_ignorable,
        path.expected(),
        eat_ignorable,
    ))
    .parse(context)?;

    let mut children = vec![kw];
    children.extend(l_ws);
    children.push(path);
    children.extend(r_ws);

    if let Some((alias, ws)) = import_alias.then(eat_ignorable).optional().parse(context)? {
        children.push(alias);
        children.extend(ws);
    }

    let semi = Punctuation::SEMICOLON.expected().parse(context)?;
    children.push(semi);

    Ok(node(SyntaxKind::IMPORT, children))
}

pub fn import_alias<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (kw, ws, name) = tuple((Keyword::AS, eat_ignorable, iden.expected())).parse(context)?;

    let mut children = vec![kw];
    children.extend(ws);
    children.push(name);

    Ok(node(SyntaxKind::IMPORT_ALIAS, children))
}
