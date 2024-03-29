use guano_syntax::{leaf, Child, SyntaxKind};

use crate::parsing::{
    combinators::{regex, Combinators},
    error::Res,
    ParseContext, Parser,
};

pub fn float_lazy<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    regex(super::FLOAT_REGEX)
        .map(|float| leaf(SyntaxKind::LIT_FLOAT, float))
        .parse(context)
}
