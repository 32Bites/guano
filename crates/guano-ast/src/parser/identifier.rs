use std::mem;

use convert_case::{Boundary, Case, Casing, StateConverter};
use guano_lexer::Token;

use crate::empty_error;

use super::{
    error::{ ParseError, ParseResult},
    Parse, ParseContext,
};

fn identify_casing(string: &str) -> Option<Case> {
    for case in Case::deterministic_cases() {
        if string.is_case(case) {
            return Some(case);
        }
    }
    None
}

#[derive(Debug, Clone)]
pub struct Identifier {
    value: String,
    casing: Option<Case>,
}

impl Identifier {
    pub fn new<S: Into<String>>(value: S) -> Self {
        let value = value.into();
        Self {
            casing: identify_casing(&value),
            value,
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn casing(&self) -> Option<Case> {
        self.casing
    }

    /// Avoids case-value mismatches.
    pub fn update_value<S: Into<String>>(&mut self, value: S) {
        let _ = mem::replace(self, Identifier::new(value));
    }

    /// Avoids case-value mismatches.
    pub fn update_casing(&mut self, case: Case) {
        if self.casing.map_or(true, |c| c != case) {
            self.value = self.value.to_case(case);
            self.casing = Some(case);
        }
    }
}

impl<S: Into<String>> From<S> for Identifier {
    fn from(s: S) -> Self {
        Self::new(s)
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl Casing<String> for Identifier {
    fn to_case(&self, case: Case) -> String {
        self.value.to_case(case)
    }

    fn from_case(&self, case: Case) -> StateConverter<String> {
        self.value.from_case(case)
    }

    fn with_boundaries(&self, bs: &[Boundary]) -> StateConverter<String> {
        self.value.with_boundaries(bs)
    }

    fn is_case(&self, case: Case) -> bool {
        if let Some(self_case) = self.casing {
            self_case == case
        } else {
            false
        }
    }
}

impl Parse<IdentifierError> for Identifier {
    fn parse(
        parser: &mut ParseContext,
    ) -> ParseResult<Identifier, IdentifierError> {
        match &parser.stream.read::<1>()[0] {
            Some((Token::Identifier(identifier), _)) => Ok(identifier.into()),
            Some((_, span)) => Err(ParseError::unexpected_token(span.clone())),
            None => Err(ParseError::EndOfFile),
        }
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Identifier {}

impl std::hash::Hash for Identifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

empty_error!(IdentifierError);
