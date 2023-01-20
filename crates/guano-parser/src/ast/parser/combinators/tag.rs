use crate::ast::parser::{
    error::{Error, ErrorKind, Res},
    string::Str,
    Parser, ParserContext,
};

#[derive(Debug, Clone, Copy)]
pub struct Tag {
    tag: &'static str,
}

#[inline]
pub fn tag(tag: &'static str) -> Tag {
    Tag { tag }
}

impl<'source> Parser<'source> for Tag {
    type Output = &'source Str;
    type Error = Error<'source>;

    fn parse(self, context: &mut ParserContext<'source>) -> Res<'source> {
        let amount = self.tag.len() as u32;

        if amount == 0 {
            return Ok(unsafe { Str::new_unchecked("") });
        }

        if context.can_advance(amount) {
            let start = context.position();
            context.advance(amount)?;
            let end = context.position();

            let value = &context.original()[start..end];

            if value == self.tag {
                Ok(value)
            } else {
                let kind = ErrorKind::Tag(self.tag, value.into());

                Err(Error::spanned(start..end, kind))
            }
        } else {
            let remaining_len = context.remaining_len();

            // Eat the remaining string
            context.advance(remaining_len)?;

            // Determine the amount of needed bytes
            let needed = amount - remaining_len;
            let span = context.span();

            let kind = ErrorKind::NeedBytes(needed);

            Err(Error::spanned(span, kind))
        }
    }
}
