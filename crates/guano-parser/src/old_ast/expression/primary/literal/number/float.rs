use std::sync::Arc;

use guano_syntax::{
    leaf,
    parser::{Input, Res as Result},
    AstToken, SyntaxKind,
};
use internment::ArcIntern;
use rug::ops::CompleteRound;

use crate::ast::prelude::*;

use super::integer::decimal;

pub trait FloatExt {
    fn value(&self) -> Option<Arc<rug::Float>>;
}

impl FloatExt for guano_syntax::tokens::Float {
    fn value(&self) -> Option<Arc<rug::Float>> {
        if let Some(float) = guano_interner::float(self.text()) {
            return Some(float);
        }

        let float = rug::Float::parse(self.text())
            .ok()
            .map(|f| f.complete(52))?;
        let interned = guano_interner::set_float(self.text(), float);

        Some(interned)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Float {
    Normal(ArcIntern<String>),
    Infinity,
    NaN,
}

impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Float::Normal(d) => d.fmt(f),
            Float::Infinity => f.write_str("inf"),
            Float::NaN => f.write_str("nan"),
        }
    }
}

pub fn parse(input: Span) -> Res<Lit> {
    let float_string = value(
        (),
        tuple((
            many1_count(digit1::<Span, _>),
            tag("."),
            many1_count(digit1),
        )),
    );
    let mut spanned_float = map(recognize(float_string), |span| {
        (span.to_node(), ArcIntern::from_ref(span.as_str()))
    });
    let (input, (span, float)) = spanned_float(input)?;

    Ok((input, Lit::new_float(Float::Normal(float), &span)))
}

pub fn float_literal<'a>(input: Input<'a>) -> Result<'a> {
    let (input, text) = recognize(delimited(decimal, tag("."), decimal))(input)?;
    let literal = leaf(SyntaxKind::LIT_FLOAT, text.fragment());

    Ok((input, literal))
}
