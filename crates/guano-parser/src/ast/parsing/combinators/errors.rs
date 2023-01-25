use std::borrow::Cow;

pub use super::expect::ExpectError;

#[derive(Debug, thiserror::Error)]
pub enum CombinatorError<'source> {
    #[error("Failed to match the regular expression {0:?}")]
    Regex(&'static str),
    #[error("Expected string {0:?}")]
    Tag(&'static str),
    #[error("Expect: {0}")]
    Expect(ExpectError<'source>),
    #[error("Was not supposed to parse {0:?}")]
    Not(Cow<'static, str>),
    #[error("Expected character {0:?}")]
    Char(char)
}

impl CombinatorError<'_> {
    pub fn to_static(self) -> CombinatorError<'static> {
        use CombinatorError::*;
        match self {
            Regex(re) => Regex(re),
            Tag(tag) => Tag(tag),
            Expect(e) => Expect(e.to_static()),
            Not(n) => Not(n),
            Char(c) => Char(c)
        }
    }
}

impl<'source> CombinatorError<'source> {
    pub fn re(re: &'static str) -> Self {
        Self::Regex(re)
    }

    pub fn tag(tag: &'static str) -> Self {
        Self::Tag(tag)
    }

/*     pub fn chars(amount: u32) -> Self {
        Self::NeedChars(amount)
    } */

    pub fn expect(error: ExpectError<'source>) -> Self {
        Self::Expect(error)
    }

    pub fn not(parser_name: impl Into<Cow<'static, str>>) -> Self {
        Self::Not(parser_name.into())
    }
}
