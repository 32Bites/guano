use crate::ast::parser::{Parser, ParserContext};

use super::Combinators;

#[derive(Debug, Clone, Copy)]
pub struct Between<P, Pr, S> {
    prefix: Pr,
    parser: P,
    suffix: S,
}

#[inline]
pub fn between<'source, P, Pr, S>(prefix: Pr, parser: P, suffix: S) -> Between<P, Pr, S>
where
    P: Parser<'source>,
    Pr: Parser<'source, Error = P::Error>,
    S: Parser<'source, Error = P::Error>,
{
    Between {
        parser,
        prefix,
        suffix,
    }
}

impl<'source, P, Pr, S> Parser<'source> for Between<P, Pr, S>
where
    P: Parser<'source>,
    Pr: Parser<'source, Error = P::Error>,
    S: Parser<'source, Error = P::Error>,
{
    type Output = P::Output;

    type Error = P::Error;

    #[inline]
    fn parse(self, context: &mut ParserContext<'source>) -> Result<Self::Output, Self::Error> {
        let ((_, output), _) = self
            .prefix
            .then(self.parser)
            .then(self.suffix)
            .parse(context)?;

        Ok(output)
    }
}
