use std::ops::Range;

use crate::ast::parser::{Parser, ParserContext};

#[derive(Debug, Clone, Copy)]
pub struct Spanned<P> {
    parser: P,
}

#[inline]
pub fn spanned<'source, P: Parser<'source>>(parser: P) -> Spanned<P> {
    Spanned { parser }
}

impl<'source, P> Parser<'source> for Spanned<P>
where
    P: Parser<'source>,
{
    type Output = (P::Output, Range<u32>);
    type Error = P::Error;

    #[inline]
    fn parse(self, context: &mut ParserContext<'source>) -> Result<Self::Output, Self::Error> {
        let start = context.position();
        let result = self.parser.parse(context)?;
        let end = context.position();

        Ok((result, start..end))
    }
}
