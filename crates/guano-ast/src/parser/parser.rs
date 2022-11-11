use std::{collections::HashMap, ffi::OsString, rc::Rc};

use codespan::{FileId, Files};
use owning_ref::RcRef;
use pest_derive::Parser;

use super::source_file::{SourceFile, SourceFileError};

#[derive(Clone, Debug, Parser)]
#[grammar = "grammar/guano.pest"]
pub(crate) struct InternalParser;

#[derive(Debug, Clone)]
pub struct Parser {
    pub(crate) files: Files<RcRef<str>>,
    pub(crate) syntax_trees: HashMap<FileId, SourceFile>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            files: Files::new(),
            syntax_trees: HashMap::new(),
        }
    }

    pub fn files(&self) -> &Files<RcRef<str>> {
        &self.files
    }

    pub fn file(
        &mut self,
        name: impl Into<OsString>,
        source: impl Into<Rc<str>>,
    ) -> Result<FileId, SourceFileError> {
        let source: RcRef<str> = source.into().into();
        let file_id = self.files.add(name, source.clone());
        let syntax_tree = SourceFile::parse(source)?;
        self.syntax_trees.insert(file_id, syntax_tree);

        Ok(file_id)
    }

    pub fn syntax_tree(&self, file_id: FileId) -> Option<&SourceFile> {
        self.syntax_trees.get(&file_id)
    }
}

/* #[derive(Debug, Clone)]
pub enum ParserError {
    Pest(PestError<Rule>),
    Ast(SourceFileError)
}

#[derive(Debug, Clone)]
pub enum AstError {

} */
