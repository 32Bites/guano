use guano_syntax::{consts::Keyword, Node};

use crate::parsing::{combinators::alternation, error::Res, ParseContext, Parser};

pub fn keyword<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    alternation((
        Keyword::TRUE,
        Keyword::FALSE,
        Keyword::NIL,
        Keyword::NAN,
        Keyword::INF,
    ))
    .parse(context)
}
