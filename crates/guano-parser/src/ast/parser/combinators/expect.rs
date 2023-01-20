use std::borrow::Cow;

use crate::ast::parser::{
    error::{Error, ErrorKind},
    string::Str,
    Parser, ParserContext,
};

#[derive(Debug, Clone)]
pub struct Expect<'source, P> {
    parser: P,
    message: Option<Cow<'source, Str>>,
}

#[inline]
pub fn expect<'source, P: Parser<'source>>(
    parser: P,
    message: impl Into<Cow<'source, Str>>,
) -> Expect<'source, P> {
    Expect {
        parser,
        message: Some(message.into()),
    }
}

#[inline]
pub fn expected<'source, P: Parser<'source>>(parser: P) -> Expect<'source, P> {
    Expect {
        parser,
        message: None,
    }
}

impl<'source, P> Parser<'source> for Expect<'source, P>
where
    P: Parser<'source, Error = Error<'source>>,
{
    type Output = Option<P::Output>;
    type Error = P::Error;

    fn parse(self, context: &mut ParserContext<'source>) -> Result<Self::Output, Self::Error> {
        let start_pos = context.position();
        let mut temp_context = context.clone();

        let result = self.parser.parse(&mut temp_context);
        let end_pos = temp_context.position();

        context.input = temp_context.input;

        match result {
            Ok(output) => {
                context.errors = temp_context.errors;

                Ok(Some(output))
            }
            Err(error) => {
                let error = self
                    .message
                    .clone()
                    .map(|s| ExpectError::Str(s))
                    .unwrap_or_else(|| ExpectError::Error(Box::new(error)));

                let kind = ErrorKind::Expect(error);
                let error = Error::spanned(start_pos..end_pos, kind);
                context.report_error(error);

                Ok(None)
            }
        }
    }
}

#[derive(Debug, ::thiserror::Error)]
pub enum ExpectError<'source> {
    #[error("{0}")]
    Str(Cow<'source, Str>),
    #[error("{0}")]
    Error(Box<Error<'source>>),
}

impl ExpectError<'_> {
    pub fn to_static(self) -> ExpectError<'static> {
        match self {
            ExpectError::Str(s) => ExpectError::Str(s.into_owned().into()),
            ExpectError::Error(e) => ExpectError::Error(Box::new((*e).to_static())),
        }
    }
}
