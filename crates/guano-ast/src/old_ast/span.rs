use std::ops::{Deref, Range};

use guano_span::span;
use guano_syntax::parser::Input;

use crate::ast::prelude::*;

pub type Span = span::Span<ParserState>;
pub type NodeSpan = span::Span<()>;
pub type ErrorSpan = LocatedSpan<ErrorString, ()>;

#[derive(Debug, Clone, Default)]
pub struct Spanned<T>(T, NodeSpan);

impl<T: std::hash::Hash> std::hash::Hash for Spanned<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<'a, D: ?Sized + pretty::DocAllocator<'a, ()>, T> pretty::Pretty<'a, D, ()> for &'a Spanned<T>
where
    &'a T: pretty::Pretty<'a, D, ()>,
{
    fn pretty(self, allocator: &'a D) -> pretty::DocBuilder<'a, D, ()> {
        self.value().pretty(allocator)
    }
}

impl<T: PartialEq> PartialEq for Spanned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Eq> Eq for Spanned<T> {}

impl<T: std::fmt::Display> std::fmt::Display for Spanned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: std::fmt::Display> Node for Spanned<T> {
    fn span(&self) -> &NodeSpan {
        &self.1
    }
}

impl<T> Spanned<T> {
    pub fn new(value: T, span: NodeSpan) -> Self {
        Spanned(value, span)
    }

    pub fn value(&self) -> &T {
        &self.0
    }

    pub fn span(&self) -> &NodeSpan {
        &self.1
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait SpanExt {
    fn to_bounded(&self) -> Range<usize>;
    fn to_node(&self) -> NodeSpan;
    fn to_error(&self) -> ErrorSpan;
    fn into_node(self) -> NodeSpan;
    fn into_error(self) -> ErrorSpan;
}

impl SpanExt for Span {
    fn to_bounded(&self) -> Range<usize> {
        let start = self.location_offset();
        let end = start + self.fragment().len();
        start..end
    }

    fn to_node(&self) -> NodeSpan {
        unsafe {
            LocatedSpan::new_from_raw_offset(
                self.location_offset(),
                self.location_line(),
                self.fragment().clone(),
                (),
            )
        }
        .into()
    }

    fn into_node(self) -> NodeSpan {
        self.to_node()
    }

    fn to_error(&self) -> ErrorSpan {
        unsafe {
            LocatedSpan::new_from_raw_offset(
                self.location_offset(),
                self.location_line(),
                (**self.fragment()).into(),
                (),
            )
        }
    }

    fn into_error(self) -> ErrorSpan {
        self.to_error()
    }
}

impl SpanExt for Input<'_> {
    fn to_bounded(&self) -> Range<usize> {
        let start = self.location_offset();
        let end = start + self.fragment().len();
        start..end
    }

    fn to_node(&self) -> NodeSpan {
        todo!()
    }

    fn to_error(&self) -> ErrorSpan {
        todo!()
    }

    fn into_node(self) -> NodeSpan {
        todo!()
    }

    fn into_error(self) -> ErrorSpan {
        todo!()
    }
}

impl SpanExt for NodeSpan {
    fn to_bounded(&self) -> Range<usize> {
        let start = self.location_offset();
        let end = start + self.fragment().len();
        start..end
    }

    fn to_node(&self) -> NodeSpan {
        self.clone()
    }

    fn into_node(self) -> NodeSpan {
        self
    }

    fn to_error(&self) -> ErrorSpan {
        unsafe {
            LocatedSpan::new_from_raw_offset(
                self.location_offset(),
                self.location_line(),
                (**self.fragment()).into(),
                (),
            )
        }
    }

    fn into_error(self) -> ErrorSpan {
        self.to_error()
    }
}
