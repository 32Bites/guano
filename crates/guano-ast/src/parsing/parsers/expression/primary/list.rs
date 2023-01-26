use guano_syntax::{consts::Punctuation, node, Node, SyntaxKind};

use crate::parsing::{
    combinators::{tuple, Combinators},
    error::Res,
    parsers::{
        expression::expr,
        ignorable::{eat_ignorable, IgnorableParser},
    },
    ParseContext, Parser,
};

pub fn list_expr<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    let (l_brack, body, r_brack) = tuple((
        Punctuation::LEFT_BRACK,
        list_expr_items,
        Punctuation::RIGHT_BRACK.expected(),
    ))
    .parse(context)?;

    let mut children = vec![l_brack];
    children.extend(body);
    children.push(r_brack);

    Ok(node(SyntaxKind::LIST_EXPR, children))
}

/// NOTE: Eats the surrounding whitespace and comments.
pub fn list_expr_items<'source>(context: &mut ParseContext<'source>) -> Res<'source, Vec<Node>> {
    let other_exprs = eat_ignorable.then(list_expr_item).repeated();
    let (l_ws, exprs, r_ws) = tuple((expr, other_exprs))
        .optional()
        .padded()
        .parse(context)?;

    let mut nodes = l_ws;
    if let Some((mut first_expr, other_exprs)) = exprs {
        first_expr = node(SyntaxKind::LIST_EXPR_ITEM, vec![first_expr]);
        nodes.push(first_expr);

        for (ws, expr) in other_exprs {
            nodes.extend(ws);
            nodes.push(expr);
        }
    }

    nodes.extend(r_ws);

    Ok(nodes)
}

pub fn list_expr_item<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    let (com, ws, expr) = tuple((
        Punctuation::COMMA,
        eat_ignorable,
        expr.expect("Expected expression"),
    ))
    .parse(context)?;
    let mut children = vec![com];
    children.extend(ws);
    children.push(expr);

    Ok(node(SyntaxKind::LIST_EXPR_ITEM, children))
}
