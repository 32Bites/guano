use std::{
    borrow::Cow,
    io::{self, Seek, SeekFrom},
    ops::Range,
    rc::Rc,
};

use guano_common::num::traits::ToPrimitive;

use super::{
    combinators::{regex, Combinators},
    error::{Error, ErrorKind, Res},
    string::Str,
};

#[derive(Debug, Clone, Default)]
/// The parser!
/// Attempts to mix the jobs of lexing as well as parsing together.
/// I got tired of working with parser combinators or overly complicated
/// lexer / parser implementations.
/// TODO: Write blog post on the current design of the parser
pub struct ParserContext<'source> {
    original: &'source Str,
    pub(crate) input: io::Cursor<&'source Str>,
    pub(crate) errors: Vec<Rc<Error<'source>>>,
}

impl<'source> ParserContext<'source> {
    #[inline]
    pub fn new(input: &'source Str) -> Self {
        Self {
            original: input,
            input: io::Cursor::new(input),
            errors: vec![],
        }
    }

    /// Create a parser without ensuring it's bounds.
    #[inline]
    pub unsafe fn new_str_unchecked(input: &'source str) -> Self {
        Self::new(Str::new_unchecked(input))
    }

    #[inline]
    pub fn new_str(input: &'source str) -> Option<Self> {
        Str::new(input).ok().map(Self::new)
    }

    #[inline]
    pub fn original(&self) -> &'source Str {
        self.original
    }

    #[inline]
    pub fn original_len(&self) -> u32 {
        self.original().len()
    }

    #[inline]
    pub fn reader(&mut self) -> Reader<'_, 'source> {
        Reader {
            inner: &mut self.input,
        }
    }

    pub fn position(&self) -> u32 {
        let mut pos = self.input.position().to_u32().unwrap();

        // Set the position to the length if it is larger than the length.
        if pos > self.original_len() {
            pos = self.original_len();
        }

        pos
    }

    #[inline]
    pub fn span(&self) -> Range<u32> {
        self.position()..(self.position() + 1)
    }

    #[inline]
    pub fn end_span(&self) -> Range<u32> {
        self.position()..self.original_len()
    }

    #[inline]
    pub fn source_span(&self) -> Range<u32> {
        0..self.original_len()
    }

    #[inline]
    pub fn remaining(&self) -> &'source Str {
        &self.original[self.position()..]
    }

    #[inline]
    pub fn remaining_len(&self) -> u32 {
        self.remaining().len()
    }

    #[inline]
    pub fn errors(&self) -> &[Rc<Error<'source>>] {
        &self.errors
    }

    #[inline]
    pub fn report_error(&mut self, error: impl Into<Error<'source>>) {
        self.errors.push(Rc::new(error.into()))
    }
}

impl<'source> ParserContext<'source> {
    pub fn can_advance(&self, amount: u32) -> bool {
        amount <= self.remaining_len()
    }

    pub fn advance(&mut self, amount: u32) -> Res<'source, ()> {
        if !self.can_advance(amount) {
            let needed = amount - self.remaining_len();
            let kind = ErrorKind::NeedBytes(needed);

            Err(Error::spanned(self.span(), kind))
        } else {
            self.input.seek(SeekFrom::Current(amount as i64))?;

            Ok(())
        }
    }

    pub fn raw_identifier(&mut self) -> Res<'source> {
        regex(r"^[_a-zA-Z][_0-9a-zA-Z]*").parse(self)
    }

    pub fn identifier(&mut self) -> Res<'source> {
        let (iden, span) = Self::raw_identifier.spanned().parse(self)?;

        if !guano_syntax::consts::keyword::ALL.contains(&iden.as_str()) {
            Ok(iden)
        } else {
            Err(Error::spanned(
                span,
                ErrorKind::String("Expected identifier"),
            ))
        }
    }

    pub fn eat_whitespace(&mut self) -> Res<'source, ()> {
        regex(r"\s*").parse(self)?;

        Ok(())
    }
}

pub trait Parser<'source>: Sized {
    type Output;
    type Error;

    fn parse(self, context: &mut ParserContext<'source>) -> Result<Self::Output, Self::Error>;

    fn name(&self) -> Cow<'static, str> {
        std::any::type_name::<Self>().into()
    }
}

impl<'source, F, T, E> Parser<'source> for F
where
    F: FnMut(&mut ParserContext<'source>) -> Result<T, E>,
{
    type Output = T;
    type Error = E;

    fn parse(mut self, context: &mut ParserContext<'source>) -> Result<T, E> {
        self(context)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Reader<'parser, 'source> {
    inner: &'parser mut io::Cursor<&'source Str>,
}

impl<'source> Reader<'_, 'source> {
    #[inline]
    pub fn get_ref(&self) -> &'source Str {
        self.inner.get_ref()
    }
}

impl<'source> io::Read for Reader<'_, 'source> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        self.inner.read_vectored(bufs)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.inner.read_exact(buf)
    }
}

impl<'source> io::BufRead for Reader<'_, 'source> {
    #[inline]
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt)
    }
}

impl<'source> io::Seek for Reader<'_, 'source> {
    #[inline]
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.inner.seek(pos)
    }

    #[inline]
    fn stream_position(&mut self) -> io::Result<u64> {
        self.inner.stream_position()
    }
}
