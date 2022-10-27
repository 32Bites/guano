use std::ops::Range;

use dyn_clone::{clone_trait_object, DynClone};
use guano_lexer::Token;
use itertools::{Itertools, MultiPeek};

pub trait ToSpannedToken {
    fn to_spanned_token(self) -> (Token, Option<Range<usize>>);
}

impl ToSpannedToken for Token {
    fn to_spanned_token(self) -> (Token, Option<Range<usize>>) {
        (self, None)
    }
}

impl ToSpannedToken for (Token, Range<usize>) {
    fn to_spanned_token(self) -> (Token, Option<Range<usize>>) {
        (self.0, Some(self.1))
    }
}

impl ToSpannedToken for (Token, Option<Range<usize>>) {
    fn to_spanned_token(self) -> (Token, Option<Range<usize>>) {
        self
    }
}

pub trait TokenSource: DynClone + std::fmt::Debug {
    fn next_token(&mut self) -> Option<(Token, Option<Range<usize>>)>;
}

impl<I: Iterator + DynClone + std::fmt::Debug> TokenSource for I
where
    I::Item: ToSpannedToken,
{
    fn next_token(&mut self) -> Option<(Token, Option<Range<usize>>)> {
        self.next().map(|i| i.to_spanned_token())
    }
}

impl Iterator for dyn TokenSource {
    type Item = (Token, Option<Range<usize>>);

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

clone_trait_object!(TokenSource);

#[derive(Debug, Clone)]
pub struct TokenStream {
    incoming: MultiPeek<Box<dyn TokenSource>>,
    last_token: Option<(Token, Option<Range<usize>>)>,
}

impl TokenStream {
    pub fn prepend<I: IntoIterator>(&self, iter: I) -> TokenStream
    where
        I::IntoIter: TokenSource + 'static,
    {
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

    pub fn last(&self) -> Option<&(Token, Option<Range<usize>>)> {
        self.last_token.as_ref()
    }

    pub fn last_span(&self) -> Option<&Option<Range<usize>>> {
        self.last_token.as_ref().map(|(_, s)| s)
    }

    pub fn last_token(&self) -> Option<&Token> {
        self.last_token.as_ref().map(|(t, _)| t)
    }

    pub fn peek<const N: usize>(&mut self) -> [Option<(Token, Option<Range<usize>>)>; N] {
        const INIT: Option<(Token, Option<Range<usize>>)> = None;
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
