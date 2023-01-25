use guano_syntax::{consts::Punctuation, node, Node, SyntaxKind};

use crate::ast::parsing::{
    combinators::tuple,
    error::Res,
    parsers::{expression::expr, ignorable::IgnorableParser},
    ParseContext, Parser,
};

pub fn group_expr<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    let (left_paren, (left_ws, expr, right_ws), right_paren) = tuple((
        Punctuation::LEFT_PAREN,
        expr.padded(),
        Punctuation::RIGHT_PAREN,
    ))
    .parse(context)?;

    let mut children = vec![left_paren];
    children.extend(left_ws);
    children.push(expr);
    children.extend(right_ws);
    children.push(right_paren);

    let group = node(SyntaxKind::GROUP_EXPR, children);

    Ok(group)
}