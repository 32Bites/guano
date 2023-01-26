use guano_syntax::{consts::Keyword, node, Node, SyntaxKind};

use crate::parsing::{
    combinators::Combinators,
    error::Res,
    parsers::{expression::expr, ignorable::eat_ignorable},
    ParseContext, Parser,
};

pub fn return_expr<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    let (kw, expr) = Keyword::RETURN
        .then(eat_ignorable.then(expr).optional())
        .parse(context)?;

    let mut children = vec![kw];
    if let Some((ws, expr)) = expr {
        children.extend(ws);
        children.push(expr);
    }

    Ok(node(SyntaxKind::RETURN_EXPR, children))
}

pub fn continue_expr<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    Ok(node(
        SyntaxKind::CONTINUE_EXPR,
        vec![Keyword::CONTINUE.parse(context)?],
    ))
}

pub fn break_expr<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    Ok(node(
        SyntaxKind::BREAK_EXPR,
        vec![Keyword::BREAK.parse(context)?],
    ))
}
