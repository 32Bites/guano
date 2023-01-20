use crate::ast::parser::{error::Error, string::Str, Parser, ParserContext};

#[derive(Debug, Clone, Copy)]
pub struct Bytes {
    count: u32,
}

#[inline]
pub fn bytes(byte_count: u32) -> Bytes {
    Bytes { count: byte_count }
}

impl<'source> Parser<'source> for Bytes {
    type Output = &'source Str;
    type Error = Error<'source>;

    #[inline]
    fn parse(self, context: &mut ParserContext<'source>) -> Result<Self::Output, Self::Error> {
        let start = context.position();
        context.advance(self.count)?;
        let end = context.position();

        Ok(&context.original()[start..end])
    }
}
