mod alternation;
mod between;
mod bytes;
mod chars;
mod count;
mod expect;
mod not;
mod optional;
mod or;
mod peek;
mod prefixed;
mod regex;
mod repeated;
mod spanned;
mod suffixed;
mod tag;
mod then;
mod tuple;

pub mod types {
    pub use super::alternation::Alternation;
    pub use super::between::Between;
    pub use super::chars::Chars;
    pub use super::count::Count;
    pub use super::expect::Expect;
    pub use super::not::Not;
    pub use super::optional::Optional;
    pub use super::or::Or;
    pub use super::peek::Peek;
    pub use super::prefixed::Prefixed;
    pub use super::regex::Regex;
    pub use super::repeated::Repeated;
    pub use super::spanned::Spanned;
    pub use super::suffixed::Suffixed;
    pub use super::tag::Tag;
    pub use super::then::Then;
    pub use super::tuple::Tuple;
}

pub mod traits {
    pub use super::tuple::TupleTrait;
}

pub mod errors {
    pub use super::expect::ExpectError;
}

use std::borrow::Cow;

pub use self::alternation::alternation;
pub use self::between::between;
pub use self::bytes::bytes;
pub use self::chars::chars;
pub use self::count::count;
pub use self::expect::{expect, expected};
pub use self::not::not;
pub use self::optional::optional;
pub use self::or::or;
pub use self::peek::peek;
pub use self::prefixed::prefixed;
pub use self::regex::regex;
pub use self::repeated::{at_least, repeated};
pub use self::spanned::spanned;
pub use self::suffixed::suffixed;
pub use self::tag::tag;
pub use self::then::then;
pub use self::tuple::tuple;

use super::string::Str;
use super::Parser;

use types::*;

pub trait Combinators<'source>: Parser<'source> {
    #[inline]
    fn or<P>(self, parser: P) -> Or<Self, P>
    where
        P: Parser<'source, Output = Self::Output, Error = Self::Error>,
    {
        or(self, parser)
    }

    #[inline]
    fn then<P>(self, parser: P) -> Then<Self, P>
    where
        P: Parser<'source, Error = Self::Error>,
    {
        then(self, parser)
    }

    #[inline]
    fn prefixed<P>(self, prefix: P) -> Prefixed<Self, P>
    where
        P: Parser<'source, Error = Self::Error>,
    {
        prefixed(prefix, self)
    }

    #[inline]
    fn suffixed<P>(self, suffix: P) -> Suffixed<Self, P>
    where
        P: Parser<'source, Error = Self::Error>,
    {
        suffixed(self, suffix)
    }

    #[inline]
    fn wrap<P, S>(self, prefix: P, suffix: S) -> Between<Self, P, S>
    where
        P: Parser<'source, Error = Self::Error>,
        S: Parser<'source, Error = Self::Error>,
    {
        between(prefix, self, suffix)
    }

    #[inline]
    fn not(self) -> Not<Self> {
        not(self)
    }

    #[inline]
    fn peek(self) -> Peek<Self> {
        peek(self)
    }

    #[inline]
    fn count(self, count: u32) -> Count<Self> {
        self::count(self, count)
    }

    #[inline]
    fn repeated(self) -> Repeated<Self> {
        repeated(self)
    }

    #[inline]
    fn at_least(self, at_least: u32) -> Repeated<Self> {
        self::at_least(self, at_least)
    }

    #[inline]
    fn optional(self) -> Optional<Self> {
        optional(self)
    }

    #[inline]
    fn expect(self, message: impl Into<Cow<'source, Str>>) -> Expect<'source, Self> {
        expect(self, message)
    }

    #[inline]
    fn expected(self) -> Expect<'source, Self> {
        expected(self)
    }

    #[inline]
    fn spanned(self) -> Spanned<Self> {
        spanned(self)
    }
}

impl<'source, P: Parser<'source>> Combinators<'source> for P {}
