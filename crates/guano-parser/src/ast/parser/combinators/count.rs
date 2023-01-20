use crate::ast::parser::{error::Error, Parser, ParserContext};

#[derive(Debug, Clone, Copy)]
pub struct Count<P> {
    parser: P,
    count: u32,
}

#[inline]
pub fn count<'source, P: Parser<'source>>(parser: P, count: u32) -> Count<P> {
    Count { parser, count }
}

impl<'source, P> Parser<'source> for Count<P>
where
    P: Parser<'source, Error = Error<'source>> + Clone,
{
    type Output = Vec<P::Output>;
    type Error = P::Error;

    #[inline]
    fn parse(self, context: &mut ParserContext<'source>) -> Result<Self::Output, Self::Error> {
        let mut output = Vec::with_capacity(self.count as usize);

        for _ in 0..self.count {
            output.push(self.parser.clone().parse(context)?);
        }

        Ok(output)
    }
}
