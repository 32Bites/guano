use guano_syntax::{leaf, Node, SyntaxKind};

use crate::ast::parsing::{
    combinators::{regex, Combinators},
    error::Res,
    ParseContext, Parser,
};

pub fn integer_lazy<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    regex(super::INTEGER_REGEX)
        .map(|int| leaf(SyntaxKind::LIT_INTEGER, int))
        .parse(context)
}
