use guano_syntax::{node, Child, SyntaxKind};

use crate::parsing::{
    combinators::{alternation, Combinators},
    error::Res,
    ParseContext, Parser,
};

pub mod char;
pub mod keyword;
pub mod number;
pub mod string;

/// Parse a literal expression.
pub fn literal<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    alternation((
        keyword::keyword,
        self::char::char_lazy,
        string::string_lazy,
        number::number_lazy,
    ))
    .map(|n| node(SyntaxKind::LITERAL, vec![n]))
    .parse(context)
}
