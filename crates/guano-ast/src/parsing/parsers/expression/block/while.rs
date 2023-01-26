use guano_syntax::{consts::Keyword, node, Child, SyntaxKind};

use crate::parsing::{
    combinators::{tuple, Combinators},
    error::Res,
    parsers::{expression::expr, ignorable::IgnorableParser},
    ParseContext, Parser,
};

use super::block;

pub fn while_expr<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (kw, (l_ws, cond, r_ws), block) =
        tuple((Keyword::WHILE, expr.expected().padded(), block.expected())).parse(context)?;

    let mut children = vec![kw];
    children.extend(l_ws);
    children.push(cond);
    children.extend(r_ws);
    children.push(block);

    Ok(node(SyntaxKind::WHILE_EXPR, children))
}
