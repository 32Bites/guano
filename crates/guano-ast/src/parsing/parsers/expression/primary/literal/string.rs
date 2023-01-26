use guano_syntax::{leaf, Node, SyntaxKind};

use crate::parsing::{
    combinators::{regex, Combinators},
    error::Res,
    ParseContext, Parser,
};

use super::char::regex::STRING_LAZY;

pub fn string_lazy<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    regex(STRING_LAZY)
        .map(|text| leaf(SyntaxKind::LIT_STRING, text))
        .parse(context)
}
