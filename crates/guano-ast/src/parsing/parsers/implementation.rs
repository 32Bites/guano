use guano_syntax::{consts::Keyword, node, Child, SyntaxKind};

use crate::parsing::{
    combinators::{tuple, Combinators},
    error::Res,
    ParseContext, Parser,
};

use super::{
    declaration::function::funcs,
    ignorable::eat_ignorable,
    symbols::{path::path, ty::ty},
};

pub fn implementation<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (kw, l_ws, proto, ty, r_ws, body) = tuple((
        Keyword::IMPL,
        eat_ignorable,
        impl_proto.then(eat_ignorable).optional(),
        ty.expected(),
        eat_ignorable,
        impl_body.expected(),
    ))
    .parse(context)?;

    let mut children = vec![kw];
    children.extend(l_ws);

    if let Some((proto, ws)) = proto {
        children.push(proto);
        children.extend(ws);
    }

    children.push(ty);
    children.extend(r_ws);
    children.push(body);

    Ok(node(SyntaxKind::IMPL, children))
}

pub fn impl_proto<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (path, ws, kw) = tuple((path, eat_ignorable, Keyword::ON)).parse(context)?;
    let mut children = vec![path];

    children.extend(ws);
    children.push(kw);

    Ok(node(SyntaxKind::IMPL_PROTO, children))
}

#[inline]
pub fn impl_body<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    funcs(SyntaxKind::IMPL_BODY)(context)
}
