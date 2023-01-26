use guano_syntax::{leaf, Child, SyntaxKind};

use crate::parsing::{
    combinators::{regex, Combinators},
    error::Res,
    ParseContext, Parser,
};

pub fn integer_lazy<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    regex(super::INTEGER_REGEX)
        .map(|int| leaf(SyntaxKind::LIT_INTEGER, int))
        .parse(context)
}
