use internment::ArcIntern;

use crate::ast::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
