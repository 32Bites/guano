use std::{borrow::Cow, io, ops::Range};

use super::{combinators::errors::ExpectError, string::Str};

pub type Res<'source, O = &'source Str> = Result<O, Error<'source>>;

#[derive(Debug)]
pub struct Error<'source> {
    pub span: Option<Range<u32>>,
    pub kind: ErrorKind<'source>,
}

impl<'source> Error<'source> {
    #[inline]
    pub fn unspanned(kind: impl Into<ErrorKind<'source>>) -> Self {
        Self {
            span: None,
            kind: kind.into(),
        }
    }

    #[inline]
    pub fn spanned(span: Range<u32>, kind: impl Into<ErrorKind<'source>>) -> Self {
        Self {
            span: Some(span),
            kind: kind.into(),
        }
    }

    #[inline]
    pub fn to_static(self) -> Error<'static> {
        Error {
            span: self.span,
            kind: self.kind.to_static(),
        }
    }
}

impl<'source, T: Into<ErrorKind<'source>>> From<T> for Error<'source> {
    #[inline]
    fn from(t: T) -> Self {
        Error::unspanned(t)
    }
}

impl std::error::Error for Error<'_> {}

impl std::fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(span) = &self.span {
            write!(f, "Error @ {}..{}: {}", span.start, span.end, self.kind)
        } else {
            write!(f, "Error: {}", self.kind)
        }
    }
}

#[derive(Debug, ::thiserror::Error)]
pub enum ErrorKind<'source> {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("Failed to match the regular expression {0:?}")]
    Regex(&'static str),
    #[error("Expected {0:?}, found {1:?}.")]
    Tag(&'static str, Cow<'source, Str>),
    #[error("Need {0} more characters.")]
    NeedChars(u32),
    #[error("Need {0} more bytes.")]
    NeedBytes(u32),
    #[error("Expect: {0}")]
    Expect(ExpectError<'source>),
    #[error("Was not supposed to parse {0:?}")]
    Not(Cow<'static, str>),
    #[error("{0}")]
    String(&'static str),
}

impl ErrorKind<'_> {
    #[inline]
    pub fn to_static(self) -> ErrorKind<'static> {
        use ErrorKind::*;
        match self {
            Io(i) => Io(i),
            Regex(r) => Regex(r.clone()),
            Tag(e, f) => Tag(e, f.into_owned().into()),
            NeedChars(n) => NeedChars(n),
            NeedBytes(n) => NeedBytes(n),
            Expect(e) => Expect(e.to_static()),
            Not(n) => Not(n),
            String(s) => String(s),
        }
    }
}
