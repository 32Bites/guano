use guano_syntax::parser::{keyword::*, Input, Res as Result};

use crate::ast::{prelude::*, symbol::iden};

pub fn parse(input: Span) -> Res<Lit> {
    map_opt(Keyword::parse, |k| k.to_literal())(input)
}

pub fn keyword<'a>(input: Input<'a>) -> Result<'a> {
    alt((
        kw_nil(iden::parse_raw),
        kw_false(iden::parse_raw),
        kw_true(iden::parse_raw),
        kw_inf(iden::parse_raw),
        kw_nan(iden::parse_raw),
    ))(input)
}
