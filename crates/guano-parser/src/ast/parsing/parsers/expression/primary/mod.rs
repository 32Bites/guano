use guano_syntax::{node, Node, SyntaxKind};

use crate::ast::parsing::{
    combinators::{alternation, Combinators},
    error::Res,
    parsers::symbols::path,
    ParseContext, Parser,
};

pub mod group;
pub mod list;
pub mod literal;
// pub mod this;

pub fn primary<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    alternation((literal::literal, path::path, group::group_expr))
        .map(|n| node(SyntaxKind::EXPR, vec![n]))
        .parse(context)
}
