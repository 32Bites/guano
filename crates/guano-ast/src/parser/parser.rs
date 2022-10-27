use codespan::{FileId, Files};
use std::{collections::HashMap, ffi::OsString, sync::Arc};

use guano_lexer::{
    logos::Logos,
    SpannedLexer, Token,
};

use super::{
    error::ParseResult,
    source_file::{SourceFile, SourceFileError},
    token_stream::{TokenSource, TokenStream},
};

/* pub type NodeId = usize;

#[derive(Debug, Clone)]
pub enum NodeValue {
    Expression,
    Block,
    Type,
    Statement,
    Function,
    Operator,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub value: Option<NodeValue>,
    pub span: Span,
}
 */
#[derive(Debug, Clone)]
pub struct ParseContext {
    pub(crate) file_id: FileId,
    pub(crate) stream: TokenStream,
    pub(crate) simplified_expressions: bool,
    pub(crate) depth: usize,
    pub(crate) max_depth: usize,
    // pub(crate) e: usize,
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
            // e: Default::default(),
        }
    }

    pub fn prepend<I: IntoIterator>(&self, iter: I) -> Option<ParseContext>
    where
        I::IntoIter: TokenSource + 'static,
    {
        if self.max_depth > self.depth {
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

    pub fn append<I: IntoIterator>(&self, iter: I) -> Option<ParseContext>
    where
        I::IntoIter: TokenSource + 'static,
    {
        if self.max_depth > self.depth {
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
}

impl Parser {
    pub fn new(simplified_expressions: bool) -> Self {
        Self {
            simplified_expressions,
            files: Files::new(),
            parsed: HashMap::new(),
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
            ParseContext::from_stream(file_id, token_stream, self.simplified_expressions, 0, 1);

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
