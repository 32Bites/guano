use std::ops::Deref;

use super::prelude::*;

#[derive(Debug, Clone)]
pub struct Error(pub NodeSpan, pub ErrorKind);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.1 {
            ErrorKind::Str(s) => s.fmt(f),
            ErrorKind::NomError(n) => n.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub enum ErrorKind {
    Str(String),
    NomError(NomError<ErrorSpan>),
}

impl Clone for ErrorKind {
    fn clone(&self) -> Self {
        match self {
            Self::Str(string) => Self::Str(string.clone()),
            Self::NomError(error) => {
                Self::NomError(NomError::new(error.input.clone(), error.code.clone()))
            }
        }
    }
}

impl From<&NomError<Span>> for ErrorKind {
    fn from(e: &NomError<Span>) -> Self {
        let error = NomError::new(e.input.clone().into_error(), e.code);
        ErrorKind::NomError(error)
    }
}

impl From<String> for ErrorKind {
    fn from(s: String) -> Self {
        Self::Str(s)
    }
}

impl From<&str> for ErrorKind {
    fn from(s: &str) -> Self {
        Self::Str(s.to_owned())
    }
}

#[derive(Debug, Clone)]
pub struct ErrorString(pub String);

impl ErrorString {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsBytes for ErrorString {
    fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Deref for ErrorString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl From<&str> for ErrorString {
    fn from(s: &str) -> Self {
        ErrorString(s.to_owned())
    }
}

impl From<String> for ErrorString {
    fn from(s: String) -> Self {
        ErrorString(s)
    }
}

impl From<ErrorString> for String {
    fn from(s: ErrorString) -> Self {
        s.0
    }
}

impl std::fmt::Display for ErrorString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
