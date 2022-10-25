use codespan::{FileId, Files};
use std::{collections::HashMap, ffi::OsString, ops::Range, sync::Arc, vec::IntoIter};

use guano_lexer::{logos::Logos, SpannedLexer, Token};
use itertools::{Itertools, MultiPeek};

use super::{
    error::ParseResult,
    source_file::{SourceFile, SourceFileError},
};

#[derive(Debug)]
pub struct TokenStream {
    incoming: MultiPeek<IntoIter<(Token, Option<Range<usize>>)>>,
}

impl TokenStream {
    pub fn prepend<II: IntoIterator<Item = (Token, Option<Range<usize>>)>>(
        &self,
        iter: II,
    ) -> TokenStream {
        iter.into_iter().chain(self.incoming.clone()).into()
    }

    pub fn append<II: IntoIterator<Item = (Token, Option<Range<usize>>)>>(
        &self,
        iter: II,
    ) -> TokenStream {
        self.incoming.clone().chain(iter.into_iter()).into()
    }

    pub fn reset_peek(&mut self) {
        self.incoming.reset_peek()
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
            if read[i].is_none() {
                break;
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

impl<I: IntoIterator<Item = (Token, Option<Range<usize>>)>> From<I> for TokenStream {
    fn from(iter: I) -> Self {
        TokenStream {
            incoming: iter.into_iter().collect::<Vec<_>>().into_iter().multipeek(),
        }
    }
}

#[derive(Debug)]
pub struct ParseContext {
    pub(crate) file_id: FileId,
    pub(crate) stream: TokenStream,
    pub(crate) simplified_expressions: bool,
    pub(crate) depth: usize,
    pub(crate) max_depth: usize,
}

impl ParseContext {
    pub fn from_stream(
        file_id: FileId,
        stream: TokenStream,
        simplified_expressions: bool,
        depth: usize,
        max_depth: usize,
    ) -> ParseContext {
        ParseContext {
            file_id,
            stream,
            simplified_expressions,
            depth,
            max_depth,
        }
    }

    pub fn prepend<II: IntoIterator<Item = (Token, Option<Range<usize>>)>>(
        &self,
        iter: II,
    ) -> Option<ParseContext> {
        if self.max_depth >= self.depth + 1 {
            Some(ParseContext::from_stream(
                self.file_id,
                self.stream.prepend(iter),
                self.simplified_expressions,
                self.depth + 1,
                self.max_depth,
            ))
        } else {
            None
        }
    }

    pub fn append<II: IntoIterator<Item = (Token, Option<Range<usize>>)>>(
        &self,
        iter: II,
    ) -> Option<ParseContext> {
        if self.max_depth >= self.depth + 1 {
            Some(ParseContext::from_stream(
                self.file_id,
                self.stream.append(iter),
                self.simplified_expressions,
                self.depth + 1,
                self.max_depth,
            ))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    pub(crate) files: Files<Arc<str>>,
    pub(crate) parsed: HashMap<FileId, SourceFile>,
    pub(crate) simplified_expressions: bool,
    // pub(crate) state: Option<ParseContext<NewSpannedLexer<Token>>>,
}

impl Parser {
    pub fn new(simplified_expressions: bool) -> Self {
        Self {
            simplified_expressions,
            files: Files::new(),
            parsed: HashMap::new(),
            // state: None,
        }
    }

    pub fn add_file(
        &mut self,
        file_name: impl Into<OsString>,
        source: impl Into<Arc<str>>,
    ) -> FileId {
        self.files.add(file_name, source.into())
    }

    pub fn parse_file<P: Parse<E, T>, E: std::error::Error, T>(
        &mut self,
        file_name: impl Into<OsString>,
        source: impl Into<Arc<str>>,
    ) -> (FileId, ParseResult<T, E>) {
        let source: Arc<str> = source.into();
        let file_id = self.add_file(file_name, source.clone());

        let token_stream = SpannedLexer::new(Token::lexer(&source)).into();

        let mut context =
            ParseContext::from_stream(file_id, token_stream, self.simplified_expressions, 0, 100);

        let result = P::parse(&mut context);

        (file_id, result)
    }

    pub fn file(
        &mut self,
        file_name: impl Into<OsString>,
        source: impl Into<Arc<str>>,
    ) -> (FileId, ParseResult<&SourceFile, SourceFileError>) {
        let (file_id, result) = self.parse_file::<SourceFile, _, _>(file_name, source);
        let result = result.map(|s| {
            self.parsed.insert(file_id, s);
            self.parsed.get(&file_id).unwrap()
        });

        (file_id, result)
    }
}

pub trait Parse<E: std::error::Error, T = Self> {
    fn parse(context: &mut ParseContext) -> ParseResult<T, E>;
}
