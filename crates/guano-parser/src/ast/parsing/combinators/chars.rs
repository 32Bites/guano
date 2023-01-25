use guano_common::rowan::TextRange;

use crate::ast::parsing::{error::Error, ParseContext, Parser};

#[derive(Debug, Clone, Copy)]
pub struct Chars {
    count: usize,
}

#[inline]
pub fn chars(character_count: usize) -> Chars {
    Chars {
        count: character_count,
    }
}

impl<'source> Parser<'source> for Chars {
    type Output = &'source str;
    type Error = Error<'source>;

    #[inline]
    fn parse(self, context: &mut ParseContext<'source>) -> Result<Self::Output, Self::Error> {
        let start_pos = context.position();
        let end_pos = context.advance_char(self.count)?;

        Ok(&context.source()[TextRange::new(start_pos, end_pos)])
    }
}
