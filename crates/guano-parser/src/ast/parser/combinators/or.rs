use crate::ast::parser::{Parser, ParserContext};

use super::Combinators;

#[derive(Debug, Clone, Copy)]
pub struct Or<P1, P2> {
    first: P1,
    second: P2,
}

#[inline]
pub fn or<'source, P1: Parser<'source>, P2: Parser<'source>>(first: P1, second: P2) -> Or<P1, P2> {
    Or { first, second }
}

impl<'source, P1, P2> Parser<'source> for Or<P1, P2>
where
    P1: Parser<'source>,
    P2: Parser<'source, Output = P1::Output, Error = P1::Error>,
{
    type Output = P1::Output;
    type Error = P1::Error;

    #[inline]
    fn parse(self, context: &mut ParserContext<'source>) -> Result<Self::Output, Self::Error> {
        if let Some(value) = self.first.optional().parse(context)? {
            Ok(value)
        } else {
            self.second.parse(context)
        }
    }
}
