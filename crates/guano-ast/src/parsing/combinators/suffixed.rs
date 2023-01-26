use crate::parsing::{ParseContext, Parser};

use super::Combinators;

#[derive(Debug, Clone, Copy)]
pub struct Suffixed<P, S> {
    parser: P,
    suffix: S,
}

#[inline]
pub fn suffixed<'source, P, S>(parser: P, suffix: S) -> Suffixed<P, S>
where
    P: Parser<'source>,
    S: Parser<'source, Error = P::Error>,
{
    Suffixed { parser, suffix }
}

impl<'source, P, T> Parser<'source> for Suffixed<P, T>
where
    P: Parser<'source>,
    T: Parser<'source, Error = P::Error>,
{
    type Output = P::Output;
    type Error = P::Error;

    #[inline]
    fn parse(self, context: &mut ParseContext<'source>) -> Result<Self::Output, Self::Error> {
        let (output, _) = self.parser.then(self.suffix).parse(context)?;

        Ok(output)
    }
}
