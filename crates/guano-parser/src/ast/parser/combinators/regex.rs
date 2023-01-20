use crate::ast::parser::{
    error::{Error, ErrorKind},
    string::Str,
    Parser, ParserContext, REGEX_REGISTRY,
};

#[derive(Debug, Clone, Copy)]
pub struct Regex {
    re: &'static str,
}

#[inline]
pub fn regex(re: &'static str) -> Regex {
    Regex { re }
}

impl<'source> Parser<'source> for Regex {
    type Output = &'source Str;
    type Error = Error<'source>;

    fn parse(self, context: &mut ParserContext<'source>) -> Result<Self::Output, Self::Error> {
        let regex = REGEX_REGISTRY.get(self.re).clone().unwrap();

        if let Some(matched) = regex.find(context.remaining().as_str()) {
            let start_pos = context.position();

            let length = matched.as_str().len();
            context.advance(length as u32)?;
            let end_pos = context.position();

            Ok(&context.original()[start_pos..end_pos])
        } else {
            let kind = ErrorKind::Regex(self.re);
            Err(Error::spanned(context.span(), kind))
        }
    }
}
