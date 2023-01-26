use std::borrow::Cow;

use guano_common::rowan::TextRange;
use guano_syntax::{leaf, Child, SyntaxKind};

use crate::parsing::{error::Error, ParseContext, Parser};

use super::errors::CombinatorError;

#[derive(Debug, Clone)]
pub struct Expect<'source, P> {
    parser: P,
    message: Option<Cow<'source, str>>,
}

#[inline]
pub fn expect<'source, P>(parser: P, message: impl Into<Cow<'source, str>>) -> Expect<'source, P>
where
    P: Parser<'source, Output = Child, Error = Error<'source>>,
{
    Expect {
        parser,
        message: Some(message.into()),
    }
}

#[inline]
pub fn expected<'source, P>(parser: P) -> Expect<'source, P>
where
    P: Parser<'source, Output = Child, Error = Error<'source>>,
{
    Expect {
        parser,
        message: None,
    }
}

impl<'source, P> Parser<'source> for Expect<'source, P>
where
    P: Parser<'source, Error = Error<'source>, Output = Child>,
{
    type Output = Child;
    type Error = Error<'source>;

    fn parse(self, context: &mut ParseContext<'source>) -> Result<Self::Output, Self::Error> {
        let start_pos = context.position();
        let mut temp_context = context.clone();

        let result = self.parser.parse(&mut temp_context);
        let end_pos = temp_context.position();

        *context.position_mut() = temp_context.position();

        match result {
            Ok(output) => {
                *context.errors_mut() = temp_context.into_errors();

                Ok(output)
            }
            Err(error) => {
                let error = self
                    .message
                    .clone()
                    .map(|s| ExpectError::Str(s))
                    .unwrap_or_else(|| ExpectError::Error(Box::new(error)));

                let kind = CombinatorError::Expect(error);
                let range = TextRange::new(start_pos, end_pos);
                let error = Error::spanned(range.clone(), kind);
                context.report_error(error);

                Ok(leaf(SyntaxKind::ERROR, &context.source()[range]))
            }
        }
    }
}

#[derive(Debug, ::thiserror::Error)]
pub enum ExpectError<'source> {
    #[error("{0}")]
    Str(Cow<'source, str>),
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
