use guano_syntax::{consts::Keyword, node, Node, SyntaxKind};

use crate::parsing::{
    combinators::{tuple, Combinators},
    error::Res,
    parsers::{
        expression::expr,
        ignorable::{eat_ignorable, IgnorableParser},
    },
    ParseContext, Parser,
};

use super::block;

pub fn if_expr<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    let (kw, (l_ws, cond, r_ws), block) =
        tuple((Keyword::IF, expr.expected().padded(), block.expected())).parse(context)?;

    let mut children = vec![kw];
    children.extend(l_ws);
    children.push(cond);
    children.extend(r_ws);
    children.push(block);

    if let Some((ws, else_block)) = eat_ignorable.then(else_block).optional().parse(context)? {
        children.extend(ws);
        children.push(else_block);
    }

    Ok(node(SyntaxKind::IF_EXPR, children))
}

pub fn else_block<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    let (kw, ws, block) = tuple((
        Keyword::ELSE,
        eat_ignorable,
        block.or(if_expr).expect("Expected block or if expression"),
    ))
    .parse(context)?;

    let mut children = vec![kw];
    children.extend(ws);
    children.push(block);

    Ok(node(SyntaxKind::ELSE_BLOCK, children))
}
