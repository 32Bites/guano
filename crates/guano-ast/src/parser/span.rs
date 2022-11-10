use std::{
    ops::{Add, Range},
    rc::Rc,
};

use super::parser::Rule;
use owning_ref::RcRef;
use pest::{iterators::Pair, Span as PestSpan};

#[derive(Clone)]
pub struct Span {
    input: RcRef<str>,
    string: RcRef<str>,
    start: usize,
    end: usize,
}

impl Span {
    pub fn from_pest_span(span: PestSpan, input: RcRef<str>) -> Self {
        Self {
            input: input.clone(),
            string: input.map(|s| &s[span.start()..span.end()]),
            start: span.start(),
            end: span.end(),
        }
    }

    pub fn as_str(&self) -> &RcRef<str> {
        &self.string
    }

    pub fn as_range(&self) -> Range<usize> {
        self.start..self.end
    }

    pub fn source(&self) -> &RcRef<str> {
        &self.input
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn combine(&self, other: &Span) -> Self {
        Self {
            input: self.input.clone(),
            start: self.start,
            end: other.end,
            string: self.input.clone().map(|s| &s[self.start..other.end]),
        }
    }
}

impl Add for &Span {
    type Output = Span;

    fn add(self, rhs: &Span) -> Self::Output {
        self.combine(rhs)
    }
}

impl std::fmt::Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Span")
            .field("string", &self.string.as_ref())
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct SpanStr {
    span: Span,
}

impl SpanStr {
    pub fn from_pest_span(span: PestSpan, input: RcRef<str>) -> Self {
        Span::from_pest_span(span, input).into()
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn value(&self) -> &RcRef<str> {
        self.span.as_str()
    }
}

impl From<Span> for SpanStr {
    fn from(span: Span) -> Self {
        Self { span }
    }
}

impl From<SpanStr> for String {
    fn from(string: SpanStr) -> Self {
        string.to_string()
    }
}

impl std::hash::Hash for SpanStr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value().hash(state)
    }
}

impl PartialEq for SpanStr {
    fn eq(&self, other: &Self) -> bool {
        self.value().as_ref() == other.value().as_ref()
    }
}

impl Eq for SpanStr {}

impl std::fmt::Display for SpanStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value().fmt(f)
    }
}

pub trait IntoSpan: Sized {
    fn into_span(self, input: RcRef<str>) -> Span;

    fn into_span_str(self, input: RcRef<str>) -> SpanStr {
        self.into_span(input).into()
    }
}

impl IntoSpan for PestSpan<'_> {
    fn into_span(self, input: RcRef<str>) -> Span {
        Span::from_pest_span(self, input)
    }
}

impl IntoSpan for Pair<'_, Rule> {
    fn into_span(self, input: RcRef<str>) -> Span {
        self.as_span().into_span(input)
    }
}
