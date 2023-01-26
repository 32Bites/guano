use crate::parsing::{ParseContext, Parser};

#[derive(Debug, Clone, Copy)]
pub struct Map<P, F> {
    parser: P,
    func: F,
}

#[inline]
pub fn map<'source, P, F, T>(parser: P, func: F) -> Map<P, F>
where
    P: Parser<'source>,
    F: FnMut(P::Output) -> T,
{
    Map { parser, func }
}

impl<'source, P, F, T> Parser<'source> for Map<P, F>
where
    P: Parser<'source>,
    F: FnMut(P::Output) -> T,
{
    type Output = T;
    type Error = P::Error;

    fn parse(mut self, context: &mut ParseContext<'source>) -> Result<Self::Output, Self::Error> {
        let output = self.parser.parse(context)?;

        Ok((self.func)(output))
    }
}
