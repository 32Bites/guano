use std::{borrow::Cow, rc::Rc};

use guano_common::rowan::{TextLen, TextRange, TextSize};
use guano_syntax::{Child, SyntaxNode};

use super::{
    error::{Error, ErrorKind, Res},
};

#[derive(Debug, Clone, Default)]
/// Contains the state of a parser.
/// This is the structure that all parsers,
/// and combinators operate on
/// to create a syntax tree.
pub struct ParseContext<'source> {
    source: &'source str,
    position: TextSize,
    errors: Vec<Rc<Error<'source>>>,
}

impl<'source> ParseContext<'source> {
    /// Create a new parser context from
    /// the source string.
    /// TODO: Ensure that the string is indexable by Rowan.
    #[inline]
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            position: 0.into(),
            errors: vec![],
        }
    }

    /// Parse the remaining input
    /// using a provided parser.
    #[inline]
    pub fn parse<P>(&mut self, parser: P) -> Result<P::Output, P::Error>
    where
        P: Parser<'source>,
    {
        parser.parse(self)
    }

    /// Parse the remaining input using a provided parser,
    /// and create a SyntaxNode if possible.
    #[inline]
    pub fn parse_ast<P>(&mut self, parser: P) -> Result<Option<SyntaxNode>, P::Error>
    where
        P: Parser<'source, Output = Child>,
    {
        parser
            .parse(self)
            .map(|c| c.into_node().map(|n| SyntaxNode::new_root(n)))
    }

    /// The entire source string.
    #[inline]
    pub fn source(&self) -> &'source str {
        self.source
    }

    /// The current position in the input.
    #[inline]
    pub fn position(&self) -> TextSize {
        self.position
    }

    /// Are we done parsing?
    #[inline]
    pub fn is_eof(&self) -> bool {
        self.remaining().len() == 0
    }

    /// Return the span of the current position.
    #[inline]
    pub fn span(&self) -> TextRange {
        TextRange::at(self.position(), 1.into())
    }

    /// Return the span of the entire source.
    #[inline]
    pub fn source_span(&self) -> TextRange {
        TextRange::up_to(self.source.text_len())
    }

    /// Return the remaining input
    #[inline]
    pub fn remaining(&self) -> &'source str {
        &self.source[(u32::from(self.position()) as usize)..]
    }

    /// Return the captured error list.
    #[inline]
    pub fn errors(&self) -> &[Rc<Error<'source>>] {
        &self.errors
    }

    /// Report an error
    #[inline]
    pub fn report_error(&mut self, error: impl Into<Error<'source>>) {
        self.errors.push(Rc::new(error.into()))
    }
}

impl<'source> ParseContext<'source> {
    /// Be careful with this, can mess up the state.
    #[inline]
    pub fn position_mut(&mut self) -> &mut TextSize {
        &mut self.position
    }

    /// Return a mutable reference to the
    /// captured error list.
    #[inline]
    pub fn errors_mut(&mut self) -> &mut Vec<Rc<Error<'source>>> {
        &mut self.errors
    }

    /// Consume self and return the captured errors.
    #[inline]
    pub fn into_errors(self) -> Vec<Rc<Error<'source>>> {
        self.errors
    }
}

impl<'source> ParseContext<'source> {
    /// Advance the position by a n bytes.
    pub fn advance_byte(&mut self, byte_amount: usize) -> Res<'source, TextSize> {
        if byte_amount as usize <= self.remaining().len() {
            let position = usize::from(self.position) + byte_amount;

            if !self.source().is_char_boundary(position) {
                let kind = ErrorKind::InvalidPosition(position as u32);
                Err(Error::spanned(self.span(), kind))
            } else {
                self.position = (position as u32).into();
                Ok(self.position)
            }
        } else {
            let needed = byte_amount - self.remaining().len();
            let kind = ErrorKind::NeedBytes(needed as u32);
            Err(Error::spanned(self.span(), kind))
        }
    }

    /// Advance the position by n characters.
    pub fn advance_char(&mut self, mut char_amount: usize) -> Res<'source, TextSize> {
        let mut byte_amount = 0;

        for ch in self.remaining().chars().take(char_amount) {
            char_amount -= 1;
            byte_amount += ch.len_utf8();
        }

        if char_amount == 0 {
            self.advance_byte(byte_amount)
        } else {
            let kind = ErrorKind::NeedChars(char_amount as u32);

            Err(Error::spanned(self.span(), kind))
        }
    }
}

pub trait Parser<'source>: Sized {
    type Output;
    type Error;

    fn parse(self, context: &mut ParseContext<'source>) -> Result<Self::Output, Self::Error>;

    #[inline]
    fn parse_ast(
        self,
        context: &mut ParseContext<'source>,
    ) -> Result<Option<SyntaxNode>, Self::Error>
    where
        Self: Parser<'source, Output = Child>,
    {
        context.parse_ast(self)
    }

    #[inline]
    fn name(&self) -> Cow<'static, str> {
        std::any::type_name::<Self>().into()
    }
}

impl<'source, F, T, E> Parser<'source> for F
where
    F: FnMut(&mut ParseContext<'source>) -> Result<T, E>,
{
    type Output = T;
    type Error = E;

    fn parse(mut self, context: &mut ParseContext<'source>) -> Result<T, E> {
        self(context)
    }
}
