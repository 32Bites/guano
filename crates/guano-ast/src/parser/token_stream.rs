use std::ops::Range;

use dyn_clone::{clone_trait_object, DynClone};
use guano_lexer::Token;
use itertools::{Itertools, MultiPeek};
use serde::{Deserialize, Serialize};

pub type Span = Option<Range<usize>>;

pub trait Spannable {
    fn get_span(&self) -> Span;
    fn slice<'a>(&self, source: &'a str) -> Option<&'a str> {
        if let Some(span) = self.get_span() {
            source.get(span)
        } else {
            None
        }
    }
}

impl Spannable for Span {
    fn get_span(&self) -> Span {
        self.clone()
    }
}

pub trait MergeSpan: Sized {
    fn merge(&self, end: &Span) -> Span;
}

impl MergeSpan for Span {
    fn merge(&self, end: &Span) -> Span {
        if let (Some(start), Some(end)) = (self, end) {
            Some(start.start..end.end)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

impl<'de, T: std::fmt::Debug + std::fmt::Display + Clone + Serialize + Deserialize<'de>>
    std::fmt::Display for Spanned<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.value, f)
    }
}

impl<T> Spannable for Spanned<T> {
    fn get_span(&self) -> Span {
        self.span.clone()
    }
}

pub trait ToSpanned<'de>: Sized + std::fmt::Debug + Clone + Deserialize<'de> + Serialize {
    fn to_spanned(self, span: Span) -> Spanned<Self> {
        Spanned { value: self, span }
    }
}

impl<'de, T> ToSpanned<'de> for T where
    T: Sized + std::fmt::Debug + Clone + Deserialize<'de> + Serialize
{
}

pub trait TokenSourceItem {
    fn token_source_item(self) -> (Token, Span);
}

impl TokenSourceItem for Token {
    fn token_source_item(self) -> (Token, Span) {
        (self, None)
    }
}

impl TokenSourceItem for (Token, Range<usize>) {
    fn token_source_item(self) -> (Token, Span) {
        (self.0, Some(self.1))
    }
}

impl TokenSourceItem for (Token, Span) {
    fn token_source_item(self) -> (Token, Span) {
        self
    }
}

pub trait TokenSource: DynClone + std::fmt::Debug {
    fn next_token(&mut self) -> Option<(Token, Span)>;
}

impl<I: Iterator + DynClone + std::fmt::Debug> TokenSource for I
where
    I::Item: TokenSourceItem,
{
    fn next_token(&mut self) -> Option<(Token, Span)> {
        self.next().map(|i| i.token_source_item())
    }
}

impl Iterator for dyn TokenSource {
    type Item = (Token, Span);

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

clone_trait_object!(TokenSource);

#[derive(Debug, Clone)]
pub struct TokenStream {
    incoming: MultiPeek<Box<dyn TokenSource>>,
    last_token: Option<(Token, Span)>,
}

impl TokenStream {
    pub fn prepend<I: IntoIterator>(&self, iter: I) -> TokenStream
    where
        I::IntoIter: TokenSource + 'static,
    {
        // TODO: Abusing boxed trait objects to avoid the responsibility of properly
        // writing my own iterators for chaining token sources to create new tokenstreams for
        // the purpose of syntax-error identification.
        // May slow down things, as it's using dynamic dispatch, and, well the heap.
        (Box::new(iter.into_iter()) as Box<dyn TokenSource>)
            .chain(Box::new(self.incoming.clone()) as Box<dyn TokenSource>)
            .into()
    }

    pub fn append<I: IntoIterator>(&self, iter: I) -> TokenStream
    where
        I::IntoIter: TokenSource + 'static,
    {
        (Box::new(self.incoming.clone()) as Box<dyn TokenSource>)
            .chain(Box::new(iter.into_iter()) as Box<dyn TokenSource>)
            .into()
    }

    pub fn reset_peek(&mut self) {
        self.incoming.reset_peek()
    }

    pub fn last(&self) -> Option<&(Token, Span)> {
        self.last_token.as_ref()
    }

    pub fn last_span(&self) -> Option<&Span> {
        self.last_token.as_ref().map(|(_, s)| s)
    }

    pub fn last_token(&self) -> Option<&Token> {
        self.last_token.as_ref().map(|(t, _)| t)
    }

    pub fn peek<const N: usize>(&mut self) -> [Option<(Token, Span)>; N] {
        const INIT: Option<(Token, Span)> = None;
        let mut peeked = [INIT; N];

        for i in 0..N {
            peeked[i] = self.incoming.peek().cloned();
            if peeked[i].is_none() {
                break;
            }
        }

        peeked
    }

    pub fn peek_token<const N: usize>(&mut self) -> [Option<Token>; N] {
        self.peek::<N>().map(|o| o.map(|(t, _)| t))
    }

    pub fn peek_span<const N: usize>(&mut self) -> [Option<Option<Range<usize>>>; N] {
        self.peek::<N>().map(|o| o.map(|(_, s)| s))
    }

    pub fn read<const N: usize>(&mut self) -> [Option<(Token, Option<Range<usize>>)>; N] {
        const INIT: Option<(Token, Option<Range<usize>>)> = None;
        let mut read = [INIT; N];

        for i in 0..N {
            read[i] = self.incoming.next();

            match &read[i] {
                Some(last_read) => self.last_token = Some(last_read.clone()),
                None => break,
            }
        }

        read
    }

    pub fn read_token<const N: usize>(&mut self) -> [Option<Token>; N] {
        self.read::<N>().map(|o| o.map(|(t, _)| t))
    }

    pub fn read_span<const N: usize>(&mut self) -> [Option<Option<Range<usize>>>; N] {
        self.read::<N>().map(|o| o.map(|(_, s)| s))
    }

    /// None means EOF.
    /// Some(None) means no EOF, but there was a failure in making a span.
    pub fn read_span_combined<const N: usize>(&mut self) -> Option<Option<Range<usize>>> {
        let spans = self.read_span::<N>();

        for span in &spans {
            match span {
                None => return None,
                Some(None) => return Some(None),
                _ => {}
            }
        }

        let span = spans.first().cloned()??.merge(&spans.last().cloned()??);

        Some(span)
    }
}

impl<I: IntoIterator> From<I> for TokenStream
where
    I::IntoIter: TokenSource + 'static,
{
    fn from(iter: I) -> Self {
        TokenStream {
            incoming: (Box::new(iter.into_iter()) as Box<dyn TokenSource>).multipeek(),
            last_token: None,
        }
    }
}
