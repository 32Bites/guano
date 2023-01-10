use crate::ast::prelude::*;

pub fn parse(input: Span) -> Res<Lit> {
    map_opt(Keyword::parse, |k| k.to_literal())(input)
}
