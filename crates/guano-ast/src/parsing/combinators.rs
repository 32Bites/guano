mod alternation;
mod ast;
mod count;
mod expect;
mod map;
mod not;
mod optional;
mod peek;
mod regex;
mod repeated;
mod spanned;
mod tag;
mod tuple;

pub mod errors;

pub mod types {
    pub use super::alternation::Alternation;
    pub use super::ast::Ast;
    pub use super::count::Count;
    pub use super::expect::Expect;
    pub use super::map::Map;
    pub use super::not::Not;
    pub use super::optional::Optional;
    pub use super::peek::Peek;
    pub use super::regex::Regex;
    pub use super::repeated::Repeated;
    pub use super::spanned::Spanned;
    pub use super::tag::Tag;
    pub use super::tuple::Tuple;
}

pub mod traits {
    pub use super::alternation::AlternationTrait;
    pub use super::tuple::TupleTrait;
}

use std::borrow::Cow;

pub use self::alternation::alternation;
use self::ast::ast;
pub use self::ast::Ast;
pub use self::count::count;
pub use self::expect::{expect, expected};
pub use self::map::map;
pub use self::not::not;
pub use self::optional::optional;
pub use self::peek::peek;
pub use self::regex::regex;
pub use self::repeated::{at_least, repeated};
pub use self::spanned::spanned;
pub use self::tag::tag;
pub use self::tuple::tuple;

use super::error::Error;
use super::Parser;

use guano_syntax::Child;
use types::*;

pub trait Combinators<'source>: Parser<'source> {
    #[inline]
    fn then<P>(self, parser: P) -> Tuple<(Self, P)>
    where
        P: Parser<'source, Error = Self::Error>,
    {
        tuple((self, parser))
    }

    #[inline]
    fn map<F, T>(self, func: F) -> Map<Self, F>
    where
        F: FnMut(Self::Output) -> T,
    {
        map(self, func)
    }

    #[inline]
    fn ast(self) -> Ast<Self>
    where
        Self: Parser<'source, Output = Child, Error = Error<'source>>,
    {
        ast(self)
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
    fn count(self, count: usize) -> Count<Self> {
        self::count(self, count)
    }

    #[inline]
    fn repeated(self) -> Repeated<Self> {
        repeated(self)
    }

    #[inline]
    fn at_least(self, at_least: usize) -> Repeated<Self> {
        self::at_least(self, at_least)
    }

    #[inline]
    fn optional(self) -> Optional<Self> {
        optional(self)
    }

    #[inline]
    fn expect(self, message: impl Into<Cow<'source, str>>) -> Expect<'source, Self>
    where
        Self: Parser<'source, Output = Child, Error = Error<'source>>,
    {
        expect(self, message)
    }

    #[inline]
    fn expected(self) -> Expect<'source, Self>
    where
        Self: Parser<'source, Output = Child, Error = Error<'source>>,
    {
        expected(self)
    }

    #[inline]
    fn spanned(self) -> Spanned<Self> {
        spanned(self)
    }
}

impl<'source, P: Parser<'source>> Combinators<'source> for P {}
