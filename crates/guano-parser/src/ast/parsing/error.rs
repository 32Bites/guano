use std::borrow::Cow;

use guano_common::rowan::TextRange;

use super::combinators::errors::CombinatorError;

pub type Res<'source, O = &'source str> = Result<O, Error<'source>>;

#[derive(Debug)]
pub struct Error<'source> {
    pub span: Option<TextRange>,
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
    pub fn spanned(span: TextRange, kind: impl Into<ErrorKind<'source>>) -> Self {
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
            write!(
                f,
                "Error @ {}..{}: {}",
                u32::from(span.start()),
                u32::from(span.end()),
                self.kind
            )
        } else {
            write!(f, "Error: {}", self.kind)
        }
    }
}

#[derive(Debug, ::thiserror::Error)]
pub enum ErrorKind<'source> {
    #[error("{0}")]
    Combinator(CombinatorError<'source>),
    #[error("Invalid string position {0}, it is not a valid UTF-8 character boundary.")]
    InvalidPosition(u32),
    #[error("Need {0} more bytes.")]
    NeedBytes(u32),
    #[error("Need {0} more characters.")]
    NeedChars(u32),
    #[error("{0}")]
    String(Cow<'static, str>),
}

impl<'source> From<CombinatorError<'source>> for ErrorKind<'source> {
    fn from(value: CombinatorError<'source>) -> Self {
        Self::Combinator(value)
    }
}

impl ErrorKind<'_> {
    #[inline]
    pub fn to_static(self) -> ErrorKind<'static> {
        use ErrorKind::*;
        match self {
            String(s) => String(s),
            Combinator(c) => Combinator(c.to_static()),
            NeedBytes(b) => NeedBytes(b),
            NeedChars(c) => NeedChars(c),
            InvalidPosition(p) => InvalidPosition(p),
        }
    }
}
