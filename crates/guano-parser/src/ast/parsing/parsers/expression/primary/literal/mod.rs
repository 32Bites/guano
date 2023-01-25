use guano_syntax::{node, Node, SyntaxKind};

use crate::ast::parsing::{
    combinators::{alternation, Combinators},
    error::Res,
    ParseContext, Parser,
};

pub mod char;
pub mod keyword;
pub mod number;
pub mod string;

pub fn literal<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    alternation((
        keyword::keyword,
        self::char::char_lazy,
        string::string_lazy,
        number::number_lazy,
    ))
    .map(|n| node(SyntaxKind::LITERAL, vec![n]))
    .parse(context)
}
