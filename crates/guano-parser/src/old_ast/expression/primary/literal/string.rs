use std::borrow::Cow;

use crate::ast::prelude::*;
use guano_interner::InternedString;
use guano_syntax::{
    leaf,
    parser::{
        error::{ErrorAccumulator, Errors},
        Input, Res as Result,
    },
    tokens::String as StringToken,
    AstToken, SyntaxKind,
};
use primary::char::text_item;

pub fn parse(input: Span) -> Res<Lit> {
    map(
        consumed(delimited(
            tag("\""),
            opt(many0(super::char::parse_text_item('"'))),
            expect(tag("\""), "Expected a \""),
        )),
        |(span, string)| {
            Lit::new_string(
                string.map(|s| s.into_iter().collect()).unwrap_or_default(),
                &span.into_node(),
            )
        },
    )(input)
}

pub fn string_literal<'a>(input: Input<'a>) -> Result<'a> {
    let (input, text) = recognize(delimited(
        tag("\""),
        opt(many0_count(super::char::text_item_lazy("\""))),
        new_expect(tag("\""), "Expected a \""),
    ))(input)?;

    let literal = leaf(SyntaxKind::LIT_STRING, text.fragment());

    Ok((input, literal))
}

pub trait StringExt {
    fn value(&self) -> Option<InternedString>;
}

impl StringExt for StringToken {
    fn value(&self) -> Option<InternedString> {
        if let Some(string) = guano_interner::string(self.text()) {
            return Some(string);
        }

        let input = LocatedSpan::new_extra(self.text(), Errors::default());
        let (_, text) = delimited(
            tag("\""),
            fold_many0(text_item("\""), String::new, |mut s, c|  {
                s.push(c);
                s
            }),
            tag("\""),
        )(input).ok()?;

        let string = guano_interner::set_string(self.text(), &text);

        Some(string)
    }
}
