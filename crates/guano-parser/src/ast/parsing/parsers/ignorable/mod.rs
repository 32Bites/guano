use guano_syntax::Node;

use crate::ast::parsing::{
    combinators::{alternation, tuple, types::Tuple, Combinators},
    error::{Error, Res},
    ParseContext, Parser,
};

pub mod comment;
pub mod whitespace;

pub fn ignorable<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    alternation((whitespace::whitespace, comment::comment)).parse(context)
}

pub fn eat_ignorable<'source>(context: &mut ParseContext<'source>) -> Res<'source, Vec<Node>> {
    ignorable.repeated().parse(context)
}

pub trait IgnorableContext<'source> {
    fn eat_ignorable(&mut self) -> Res<'source, Vec<Node>>;
}

impl<'source> IgnorableContext<'source> for ParseContext<'source> {
    fn eat_ignorable(&mut self) -> Res<'source, Vec<Node>> {
        eat_ignorable(self)
    }
}

type IgnoreFn<'source> = fn(&mut ParseContext<'source>) -> Res<'source, Vec<Node>>;

pub trait IgnorableParser<'source>: Parser<'source, Error = Error<'source>> {
    #[inline]
    fn padded(self) -> Tuple<(IgnoreFn<'source>, Self, IgnoreFn<'source>)> {
        tuple((eat_ignorable, self, eat_ignorable))
    }
}

impl<'source, P> IgnorableParser<'source> for P where P: Parser<'source, Error = Error<'source>> {}
