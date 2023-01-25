use guano_common::rowan::TextRange;

use crate::ast::parsing::{error::Error, ParseContext, Parser};

use super::errors::CombinatorError;

#[derive(Debug, Clone, Copy)]
pub struct Char {
    c: char,
}

pub fn char(c: char) -> Char {
    Char { c }
}

impl<'source> Parser<'source> for Char {
    type Output = &'source str;
    type Error = Error<'source>;

    fn parse(self, context: &mut ParseContext<'source>) -> Result<Self::Output, Self::Error> {
        let start_pos = context.position();
        if context.remaining().chars().next() == Some(self.c) {
            let end_pos = context.advance_char(1)?;
            Ok(&context.source()[TextRange::new(start_pos, end_pos)])
        } else {
            let kind = CombinatorError::Char(self.c);

            Err(Error::spanned(context.span(), kind))
        }
    }
}
