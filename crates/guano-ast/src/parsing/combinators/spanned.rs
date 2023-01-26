use guano_common::rowan::TextRange;

use crate::parsing::{ParseContext, Parser};

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
    type Output = (P::Output, TextRange);
    type Error = P::Error;

    #[inline]
    fn parse(self, context: &mut ParseContext<'source>) -> Result<Self::Output, Self::Error> {
        let start_pos = context.position();
        let result = self.parser.parse(context)?;
        let end_pos = context.position();

        Ok((result, TextRange::new(start_pos, end_pos)))
    }
}
