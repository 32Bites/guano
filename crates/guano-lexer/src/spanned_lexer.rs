use std::{ops::RangeInclusive, vec::IntoIter};

use logos::{Lexer, Logos, Source, Span as ByteSpan, SpannedIter};

use crate::token::Token;

/*
*   This is probably, most definitely, shitty.
*   And to you I say: I'm tired, leave me alone, I just need it to work.
*   Also I did this when my brain has been more or less nonfunctional, so uh yeah, imma go to sleep.
*
*   - Noah Shanaberger, October 8th 2022 - 12:27 AM.
*/

#[derive(Debug, Clone)]
pub struct NewSpannedLexer<Token>(IntoIter<(Token, Option<ByteSpan>)>);

impl NewSpannedLexer<Token> {
    pub fn new(lexer: Lexer<'_, Token>) -> Self {
        Self(
            lexer
                .spanned()
                .filter(|(t, _)| !matches!(t, Token::CommMulti(_) | Token::CommSingle(_)))
                .map(|(t, r)| (t, Some(r)))
                .collect::<Vec<_>>()
                .into_iter(),
        )
    }
}

impl<'source, Token: Logos<'source>> Iterator for NewSpannedLexer<Token> {
    type Item = (Token, Option<ByteSpan>);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[derive(Debug, Clone)]
pub struct Span {
    pub byte_span: ByteSpan,
    pub line_span: LineSpan,
}

impl Span {
    pub fn extend(&self, other: &Span) -> Span {
        Span {
            byte_span: self.byte_span.start..other.byte_span.end,
            line_span: self.line_span.extend(&other.line_span),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LineSpan {
    pub lines: RangeInclusive<usize>,
    pub start_line_column: usize,
    pub end_line_column: usize,
}

impl LineSpan {
    pub fn extend(&self, other: &LineSpan) -> LineSpan {
        LineSpan {
            lines: *self.lines.start()..=*other.lines.end(),
            start_line_column: self.start_line_column,
            end_line_column: other.end_line_column,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LineSegment<'source, S: ?Sized> {
    pub number: usize,
    pub byte_span: ByteSpan,
    pub value: &'source S,
}

pub struct SpannedLexer<'source, Token: Logos<'source>> {
    pub lexer: Lexer<'source, Token>,
    pub newlines: Vec<Option<usize>>,
}

impl<'source, Token: Logos<'source>> SpannedLexer<'source, Token> {
    pub fn new(lexer: Lexer<'source, Token>) -> Self {
        Self {
            lexer,
            newlines: vec![None],
        }
    }

    pub fn slice(&self) -> &<Token::Source as Source>::Slice {
        self.lexer.slice()
    }

    pub fn extras(&self) -> &Token::Extras {
        &self.lexer.extras
    }

    pub fn extras_mut(&mut self) -> &mut Token::Extras {
        &mut self.lexer.extras
    }

    pub fn get_line(&self, number: usize) -> Option<&<Token::Source as Source>::Slice> {
        let newlines_index = number.checked_sub(1)?;

        let start = self.newlines.get(newlines_index)?.map_or(0, |i| i + 1);
        let end = self
            .newlines
            .get(newlines_index + 1)
            .map_or(self.lexer.source().len(), |n| n.unwrap());

        self.lexer.source().slice(start..end)
    }

    pub fn line_start(&self, number: usize) -> Option<usize> {
        self.newlines
            .get(number.checked_sub(1)?)?
            .map_or(Some(0), |s| Some(s + 1))
    }

    pub fn line_end(&self, number: usize) -> Option<usize> {
        self.newlines
            .get(number)
            .map_or(Some(self.lexer.span().end), |s| *s)
    }

    pub fn lines(&self) -> Option<Vec<(usize, &<Token::Source as Source>::Slice)>> {
        let mut lines = vec![];
        let mut windows = self.newlines.windows(2).map(|w| w.try_into().unwrap());

        while let Some::<[Option<usize>; 2]>([first, second]) = windows.next() {
            let start = first.map_or(0, |s| s + 1);
            let end = second?;

            lines.push((lines.len() + 1, self.lexer.source().slice(start..end)?))
        }

        let start = self.newlines.last().cloned().flatten().map_or(0, |s| s + 1);

        let end = self.lexer.source().len();

        lines.push((lines.len() + 1, self.lexer.source().slice(start..end)?));

        Some(lines)
    }

    pub fn line_segments(
        &self,
        span: &LineSpan,
    ) -> Option<Vec<LineSegment<'source, <Token::Source as Source>::Slice>>> {
        let mut line_iter = span.lines.clone();

        let mut lines = Vec::from([{
            let number = line_iter.next()?;
            let start_index = self.line_start(number)? + span.start_line_column;
            let end_index = self.line_end(number)?;

            let byte_span = start_index..end_index;

            let value = self.lexer.source().slice(byte_span.clone())?;
            LineSegment {
                number,
                byte_span,
                value,
            }
        }]);

        while let Some(number) = line_iter.next() {
            let start_index = self.line_start(number)?;
            let end_index = self.line_end(number)?;

            let byte_span = start_index..end_index;

            let value = self.lexer.source().slice(byte_span.clone())?;
            lines.push(LineSegment {
                number,
                byte_span,
                value,
            })
        }

        let segment = lines.last_mut()?;
        let end = self.line_start(segment.number)? + span.end_line_column;

        segment.byte_span.end = end;
        segment.value = self.lexer.source().slice(segment.byte_span.clone())?;

        Some(lines)
    }

    pub fn line_number(&self) -> usize {
        self.newlines.len()
    }
}

impl<'source, Token: Logos<'source> + Clone> Clone for SpannedLexer<'source, Token>
where
    Token::Extras: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.lexer.clone())
    }
}

impl<'source, Token: Logos<'source> + std::fmt::Debug> std::fmt::Debug
    for SpannedLexer<'source, Token>
where
    Token::Extras: std::fmt::Debug,
    Token::Source: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpannedLexer")
            .field("lexer", &self.lexer)
            .field("newlines", &self.newlines)
            .finish()
    }
}

impl<'source, Token: Logos<'source> + PartialEq + PushNewlines<'source>> Iterator
    for SpannedLexer<'source, Token>
{
    type Item = (Token, Span);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let token = self.lexer.next()?;
            let start_line = self.line_number();
            let start_location = self.newlines.last().unwrap().map_or(0, |u| u + 1);

            token.push_newlines(&self.lexer, &mut self.newlines);

            if token != Token::NEWLINE {
                let byte_span = self.lexer.span();

                let end_line = self.line_number();
                let end_location = self.newlines.last().unwrap().map_or(0, |u| u + 1);

                let span = Span {
                    line_span: LineSpan {
                        lines: start_line..=end_line,
                        start_line_column: byte_span.start - start_location,
                        end_line_column: byte_span.end - end_location,
                    },

                    byte_span,
                };

                return Some((token, span));
            }
        }
    }
}

pub trait ToSpanned<'source, Token: Logos<'source>> {
    fn to_spanned(self) -> SpannedLexer<'source, Token>;
}

impl<'source, Token: Logos<'source>> ToSpanned<'source, Token> for Lexer<'source, Token> {
    fn to_spanned(self) -> SpannedLexer<'source, Token> {
        SpannedLexer::new(self)
    }
}

pub trait PushNewlines<'source>: Logos<'source> + PartialEq {
    const NEWLINE: Self;
    fn push_newlines(&self, lex: &Lexer<'source, Self>, newlines: &mut Vec<Option<usize>>)
        -> usize;
}

impl<'source> PushNewlines<'source> for Token {
    const NEWLINE: Self = Self::Newline;
    fn push_newlines(
        &self,
        lex: &Lexer<'source, Token>,
        newlines: &mut Vec<Option<usize>>,
    ) -> usize {
        match self {
            Self::Newline => {
                newlines.push(Some(lex.span().start));
                1
            }
            Self::CommMulti((_, n)) => {
                for newline in n {
                    newlines.push(Some(*newline));
                }

                n.len()
            }
            _ => 0,
        }
    }
}
