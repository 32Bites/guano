use std::{borrow::Cow, rc::Rc};

use guano_common::rowan::{TextLen, TextRange, TextSize};

use super::{
    combinators::{regex, Combinators},
    error::{Error, ErrorKind, Res},
};

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
/// Attempts to mix the jobs of lexing as well as parsing together.
/// I got tired of working with parser combinators or overly complicated
/// lexer / parser implementations.
/// TODO: Write blog post on the current design of the parser
pub struct ParseContext<'source> {
    source: &'source str,
    position: TextSize,
    errors: Vec<Rc<Error<'source>>>,
}

impl<'source> ParseContext<'source> {
    #[inline]
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            position: 0.into(),
            errors: vec![],
        }
    }

    #[inline]
    pub fn parse<P>(&mut self, parser: P) -> Result<P::Output, P::Error>
    where
        P: Parser<'source>,
    {
        parser.parse(self)
    }

    #[inline]
    pub fn source(&self) -> &'source str {
        self.source
    }

    #[inline]
    pub fn position(&self) -> TextSize {
        self.position
    }

    #[inline]
    pub fn is_eof(&self) -> bool {
        self.remaining().len() == 0
    }

    #[inline]
    pub fn span(&self) -> TextRange {
        TextRange::at(self.position(), 1.into())
    }

    #[inline]
    pub fn source_span(&self) -> TextRange {
        TextRange::up_to(self.source.text_len())
    }

    #[inline]
    pub fn remaining(&self) -> &'source str {
        &self.source[(u32::from(self.position()) as usize)..]
    }

    #[inline]
    pub fn errors(&self) -> &[Rc<Error<'source>>] {
        &self.errors
    }

    #[inline]
    pub fn report_error(&mut self, error: impl Into<Error<'source>>) {
        self.errors.push(Rc::new(error.into()))
    }

    #[inline]
    pub fn errors_mut(&mut self) -> &mut Vec<Rc<Error<'source>>> {
        &mut self.errors
    }

    #[inline]
    pub fn into_errors(self) -> Vec<Rc<Error<'source>>> {
        self.errors
    }
}

impl<'source> ParseContext<'source> {
    #[inline]
    #[doc = "Be careful with this, can mess up the state."]
    pub fn position_mut(&mut self) -> &mut TextSize {
        &mut self.position
    }
}

impl<'source> ParseContext<'source> {
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

    pub fn raw_identifier(&mut self) -> Res<'source> {
        regex(r"^[_a-zA-Z][_0-9a-zA-Z]*").parse(self)
    }

    pub fn identifier(&mut self) -> Res<'source> {
        let (iden, span) = Self::raw_identifier.spanned().parse(self)?;

        if guano_syntax::consts::Keyword::ALL
            .into_iter()
            .map(|k| k.as_str())
            .all(|s| s != iden)
        {
            Ok(iden)
        } else {
            Err(Error::spanned(
                span,
                ErrorKind::String("Expected identifier".into()),
            ))
        }
    }

    pub fn eat_whitespace(&mut self) -> Res<'source, ()> {
        regex(r"^\s*").parse(self)?;

        Ok(())
    }
}

pub trait Parser<'source>: Sized {
    type Output;
    type Error;

    fn parse(self, context: &mut ParseContext<'source>) -> Result<Self::Output, Self::Error>;

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
