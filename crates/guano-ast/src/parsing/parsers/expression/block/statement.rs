use guano_common::rowan::Language;
use guano_syntax::{consts::Punctuation, node, Child, Lang, SyntaxKind};

use crate::parsing::{
    combinators::{alternation, Combinators},
    error::Res,
    parsers::{declaration::variable::var, expression::expr, ignorable::eat_ignorable},
    ParseContext, Parser,
};

pub fn statement<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    alternation((expr_statement, var)).parse(context)
}

pub fn expr_statement<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let expr = expr(context)?;

    let is_block = match expr.as_node() {
        Some(node) => {
            let kind = Lang::kind_from_raw(node.kind());
            kind.is_block_expr()
        }
        None => false,
    };

    let mut children = vec![expr];

    if is_block {
        if let Some((ws, semi)) = eat_ignorable
            .then(Punctuation::SEMICOLON)
            .optional()
            .parse(context)?
        {
            children.extend(ws);
            children.push(semi);
        }
    } else {
        let (ws, semi) = eat_ignorable
            .then(Punctuation::SEMICOLON.expected())
            .parse(context)?;
        children.extend(ws);
        children.push(semi);
    }

    let statement = node(SyntaxKind::EXPR_STATEMENT, children);

    Ok(statement)
}
