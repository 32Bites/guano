use guano_syntax::{
    consts::{Keyword, Punctuation},
    node, Child, SyntaxKind,
};

use crate::parsing::{
    combinators::{alternation, tuple, Combinators},
    error::Res,
    parsers::{
        ignorable::{eat_ignorable, IgnorableParser},
        implementation::implementation,
        symbols::identifier::iden,
    },
    ParseContext, Parser,
};

use super::decl;

pub fn module<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (pub_kw, module, (l_ws, name, r_ws), body) = tuple((
        Keyword::PUB.then(eat_ignorable).optional(),
        Keyword::MODULE,
        iden.expected().padded(),
        module_body.expected(),
    ))
    .parse(context)?;

    let mut children = vec![];

    if let Some((kw, ws)) = pub_kw {
        children.push(kw);
        children.extend(ws);
    }
    children.push(module);
    children.extend(l_ws);
    children.push(name);
    children.extend(r_ws);
    children.push(body);

    Ok(node(SyntaxKind::MODULE, children))
}

pub fn module_body<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (l_curly, (l_ws, items, r_ws), r_curly) = tuple((
        Punctuation::LEFT_CURLY,
        module_items.padded(),
        Punctuation::RIGHT_CURLY.expected(),
    ))
    .parse(context)?;

    let mut children = vec![l_curly];
    children.extend(l_ws);
    children.extend(items);
    children.extend(r_ws);
    children.push(r_curly);

    Ok(node(SyntaxKind::MODULE_BODY, children))
}

pub fn module_items<'source>(context: &mut ParseContext<'source>) -> Res<'source, Vec<Child>> {
    let items = module_item.then(eat_ignorable).repeated().parse(context)?;
    let mut new_items = vec![];

    for (item, ws) in items {
        new_items.push(item);
        new_items.extend(ws);
    }

    Ok(new_items)
}

pub fn module_item<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    alternation((decl, implementation)).parse(context)
}
