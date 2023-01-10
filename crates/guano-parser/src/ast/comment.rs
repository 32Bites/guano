use crate::ast::prelude::*;

pub fn parse(input: Span) -> Res<()> {
    alt((single_line, block))(input)
}

pub fn single_line(input: Span) -> Res<()> {
    value((), preceded(tag("//"), not_line_ending))(input)
}

pub fn block(input: Span) -> Res<()> {
    value(
        (),
        delimited(
            tag("/*"),
            many0_count(alt((
                value((), pair(not(alt((tag("/*"), tag("*/")))), anychar)),
                value((), block),
            ))),
            expect(tag("*/"), "Expected block comment close '*/'"),
        ),
    )(input)
}
