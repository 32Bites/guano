use crate::ast::parsing::{ParseContext, Parser};

use super::Combinators;

#[derive(Debug, Clone, Copy)]
pub struct Prefixed<P, Pr> {
    prefix: Pr,
    parser: P,
}

#[inline]
pub fn prefixed<'source, P, Pr>(prefix: Pr, parser: P) -> Prefixed<P, Pr>
where
    P: Parser<'source>,
    Pr: Parser<'source, Error = P::Error>,
{
    Prefixed { parser, prefix }
}

impl<'source, P, Pr> Parser<'source> for Prefixed<P, Pr>
where
    P: Parser<'source>,
    Pr: Parser<'source, Error = P::Error>,
{
    type Output = P::Output;
    type Error = P::Error;

    #[inline]
    fn parse(self, context: &mut ParseContext<'source>) -> Result<Self::Output, Self::Error> {
        let (_, output) = self.prefix.then(self.parser).parse(context)?;

        Ok(output)
    }
}
