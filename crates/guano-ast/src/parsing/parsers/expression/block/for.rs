use guano_syntax::{consts::Keyword, node, Child, SyntaxKind};

use crate::parsing::{
    combinators::{tuple, Combinators},
    error::Res,
    parsers::{
        expression::expr,
        ignorable::{eat_ignorable, IgnorableParser},
        symbols::identifier::iden,
    },
    ParseContext, Parser,
};

use super::block;

pub fn for_expr<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (for_kw, (l_ws, iden, r_ws), in_kw) = tuple((
        Keyword::FOR,
        iden.expected().padded(),
        Keyword::IN.expected(),
    ))
    .parse(context)?;

    let mut children = vec![for_kw];
    children.extend(l_ws);
    children.push(iden);
    children.extend(r_ws);
    children.push(in_kw);

    let (expr, ws, block) =
        tuple((expr.expected(), eat_ignorable, block.expected())).parse(context)?;
    children.push(expr);
    children.extend(ws);
    children.push(block);

    Ok(node(SyntaxKind::FOR_EXPR, children))
}
