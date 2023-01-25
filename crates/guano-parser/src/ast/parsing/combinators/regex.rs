use guano_common::rowan::TextRange;

use crate::ast::parsing::{
    error::Error,
    Parser, ParseContext, regex_registry::REGEX_REGISTRY,
};

use super::errors::CombinatorError;

#[derive(Debug, Clone, Copy)]
pub struct Regex {
    re: &'static str,
}

#[inline]
pub fn regex(re: &'static str) -> Regex {
    Regex { re }
}

impl<'source> Parser<'source> for Regex {
    type Output = &'source str;
    type Error = Error<'source>;

    fn parse(self, context: &mut ParseContext<'source>) -> Result<Self::Output, Self::Error> {
        let regex = REGEX_REGISTRY.get(self.re).clone().unwrap();

        if let Some(matched) = regex.find(context.remaining()) {
            let start_pos = context.position();

            let length = matched.as_str().len();
            let end_pos = context.advance_byte(length)?;

            Ok(&context.source()[TextRange::new(start_pos, end_pos)])
        } else {
            let kind = CombinatorError::Regex(self.re);
            Err(Error::spanned(context.span(), kind))
        }
    }

    fn name(&self) -> std::borrow::Cow<'static, str> {
        format!("Regex({:?})", self.re).into()
    }
}
