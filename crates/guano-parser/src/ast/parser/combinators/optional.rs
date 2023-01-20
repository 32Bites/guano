use crate::ast::parser::{Parser, ParserContext};

#[derive(Debug, Clone, Copy)]
pub struct Optional<P> {
    parser: P,
}

#[inline]
pub fn optional<'source, P: Parser<'source>>(parser: P) -> Optional<P> {
    Optional { parser }
}

impl<'source, P> Parser<'source> for Optional<P>
where
    P: Parser<'source>,
{
    type Output = Option<P::Output>;
    type Error = P::Error;

    #[inline]
    fn parse(self, context: &mut ParserContext<'source>) -> Result<Self::Output, Self::Error> {
        let mut temp_context = context.clone();

        if let Some(value) = self.parser.parse(&mut temp_context).ok() {
            context.input = temp_context.input;
            context.errors = temp_context.errors;

            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}
