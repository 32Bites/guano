use guano_syntax::parser::{Input, Res as Result};

use crate::ast::prelude::*;

use self::{integer::integer_literal, float::float_literal};

pub mod float;
pub mod integer;

pub fn parse(input: Span) -> Res<Lit> {
    alt((float::parse, integer::parse))(input)
}

pub fn number_literal<'a>(input: Input<'a>) -> Result<'a> {
    alt((float_literal,integer_literal))(input)
}