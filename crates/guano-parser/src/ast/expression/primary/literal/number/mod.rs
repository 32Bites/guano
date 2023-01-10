use crate::ast::prelude::*;

pub mod float;
pub mod integer;

pub fn parse(input: Span) -> Res<Lit> {
    alt((float::parse, integer::parse))(input)
}
