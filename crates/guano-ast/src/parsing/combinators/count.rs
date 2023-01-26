use crate::parsing::{error::Error, ParseContext, Parser};

#[derive(Debug, Clone, Copy)]
pub struct Count<P> {
    parser: P,
    count: usize,
}

#[inline]
pub fn count<'source, P: Parser<'source>>(parser: P, count: usize) -> Count<P> {
    Count { parser, count }
}

impl<'source, P> Parser<'source> for Count<P>
where
    P: Parser<'source, Error = Error<'source>> + Clone,
{
    type Output = Vec<P::Output>;
    type Error = P::Error;

    #[inline]
    fn parse(self, context: &mut ParseContext<'source>) -> Result<Self::Output, Self::Error> {
        let mut output = Vec::with_capacity(self.count);

        for _ in 0..self.count {
            output.push(self.parser.clone().parse(context)?);
        }

        Ok(output)
    }
}
