use crate::ast::parser::{Parser, ParserContext};

#[derive(Debug, Clone, Copy)]
pub struct Then<P1, P2> {
    first: P1,
    second: P2,
}

#[inline]
pub fn then<'source, P1, P2>(first: P1, second: P2) -> Then<P1, P2>
where
    P1: Parser<'source>,
    P2: Parser<'source, Error = P1::Error>,
{
    Then { first, second }
}

impl<'source, P1, P2> Parser<'source> for Then<P1, P2>
where
    P1: Parser<'source>,
    P2: Parser<'source, Error = P1::Error>,
{
    type Output = (P1::Output, P2::Output);

    type Error = P1::Error;

    #[inline]
    fn parse(self, context: &mut ParserContext<'source>) -> Result<Self::Output, Self::Error> {
        let result = (self.first.parse(context)?, self.second.parse(context)?);

        Ok(result)
    }
}
