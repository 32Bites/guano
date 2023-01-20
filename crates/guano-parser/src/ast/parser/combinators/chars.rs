use crate::ast::parser::{
    error::{Error, ErrorKind},
    string::Str,
    Parser, ParserContext,
};
use utf8_chars::BufReadCharsExt;

#[derive(Debug, Clone, Copy)]
pub struct Chars {
    count: u32,
}

#[inline]
pub fn chars(character_count: u32) -> Chars {
    Chars {
        count: character_count,
    }
}

impl<'source> Parser<'source> for Chars {
    type Output = &'source Str;
    type Error = Error<'source>;

    #[inline]
    /// TODO: Rewrite to avoid [utf8_chars].
    fn parse(self, context: &mut ParserContext<'source>) -> Result<Self::Output, Self::Error> {
        assert_ne!(self.count, 0);

        let start = context.position();

        let iter = (0..self.count).scan(self.count, |state, _| {
            *state -= 1;
            Some(*state)
        });

        for amount in iter {
            if let None = context.reader().read_char()? {
                let end = context.position();
                return Err(Error::spanned(start..end, ErrorKind::NeedChars(amount)));
            }
        }

        let end = context.position();

        Ok(&context.original()[start..end])
    }
}
