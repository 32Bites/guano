use guano_syntax::{consts::Keyword, node, Node, SyntaxKind};

use crate::parsing::{
    combinators::{tuple, Combinators},
    error::Res,
    parsers::ignorable::eat_ignorable,
    ParseContext, Parser,
};

use super::block;

pub fn loop_expr<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    let (kw, ws, block) = tuple((Keyword::LOOP, eat_ignorable, block.expected())).parse(context)?;

    let mut children = vec![kw];
    children.extend(ws);
    children.push(block);

    let r#loop = node(SyntaxKind::LOOP_EXPR, children);
    // let expr = node(SyntaxKind::EXPR, vec![r#loop]);

    Ok(r#loop)
}
