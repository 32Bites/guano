use guano_common::rowan::TextRange;

use crate::ast::parsing::{
    error::{Error, Res},
    ParseContext, Parser,
};

use super::errors::CombinatorError;

#[derive(Debug, Clone, Copy)]
pub struct Tag {
    tag: &'static str,
}

#[inline]
pub fn tag(tag: &'static str) -> Tag {
    Tag { tag }
}

impl<'source> Parser<'source> for Tag {
    type Output = &'source str;
    type Error = Error<'source>;

    fn parse(self, context: &mut ParseContext<'source>) -> Res<'source> {
        let has_tag = context.remaining().starts_with(self.tag);

        if has_tag {
            let start_pos = context.position();
            let end_pos = context.advance_byte(self.tag.len())?;
            let value = &context.source()[TextRange::new(start_pos, end_pos)];

            Ok(value)
        } else {
            let kind = CombinatorError::Tag(self.tag);
            Err(Error::spanned(context.span(), kind))
        }
    }

    fn name(&self) -> std::borrow::Cow<'static, str> {
        format!("Tag({:?})", self.tag).into()
    }
}
