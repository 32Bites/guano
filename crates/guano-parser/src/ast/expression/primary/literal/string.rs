use crate::ast::prelude::*;

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
