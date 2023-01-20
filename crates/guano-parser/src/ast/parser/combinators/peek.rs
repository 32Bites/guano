use crate::ast::parser::{Parser, ParserContext};

#[derive(Debug, Clone, Copy)]
pub struct Peek<P> {
    parser: P,
}

#[inline]
pub fn peek<'source, P: Parser<'source>>(parser: P) -> Peek<P> {
    Peek { parser }
}

impl<'source, P> Parser<'source> for Peek<P>
where
    P: Parser<'source>,
{
    type Output = P::Output;
    type Error = P::Error;

    #[inline]
    fn parse(self, context: &mut ParserContext<'source>) -> Result<Self::Output, Self::Error> {
        let mut temp_context = context.clone();
        let result = self.parser.parse(&mut temp_context);
        context.errors = temp_context.errors;

        result
    }
}
