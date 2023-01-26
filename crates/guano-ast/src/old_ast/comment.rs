use guano_syntax::{leaf, parser::Input, SyntaxKind};

use crate::ast::prelude::*;

pub fn parse(input: Span) -> Res<()> {
    alt((single_line, block))(input)
}

pub fn single_line(input: Span) -> Res<()> {
    value((), preceded(tag("//"), not_line_ending))(input)
}

pub fn comment<'a>(input: Input<'a>) -> guano_syntax::parser::Res<'a> {
    alt((line_comment, block_comment))(input)
}

pub fn line_comment_lazy<'a>(input: Input<'a>) -> guano_syntax::parser::Res<'a, Input<'a>> {
    recognize(preceded(tag("//"), not_line_ending))(input)
}

pub fn line_comment<'a>(input: Input<'a>) -> guano_syntax::parser::Res<'a> {
    map(line_comment_lazy, |s| {
        leaf(SyntaxKind::COMMENT, s.fragment())
    })(input)
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

pub fn block_comment_lazy<'a>(input: Input<'a>) -> guano_syntax::parser::Res<'a, Input<'a>> {
    recognize(delimited(
        tag("/*"),
        many0_count(alt((
            value((), pair(not(alt((tag("/*"), tag("*/")))), anychar)),
            value((), block_comment_lazy),
        ))),
        new_expect(tag("*/"), "Expected block comment close '*/'"),
    ))(input)
}

pub fn block_comment<'a>(input: Input<'a>) -> guano_syntax::parser::Res<'a> {
    map(block_comment_lazy, |s| {
        leaf(SyntaxKind::COMMENT, s.fragment())
    })(input)
}
