use std::collections::HashMap;

use codespan::{FileId, Files};
use owning_ref::RcRef;

use crate::parser::Parser;

use self::source_file::SourceFile;

pub mod block;
pub mod declaration;
pub mod source_file;
pub mod statement;

#[derive(Debug, Clone)]
pub struct Unsweetened {
    pub(crate) files: Files<RcRef<str>>,
    pub(crate) syntax_trees: HashMap<FileId, SourceFile>,
}

impl Unsweetened {
    pub fn files(&self) -> &Files<RcRef<str>> {
        &self.files
    }

    pub fn syntax_tree(&self, file_id: FileId) -> Option<&SourceFile> {
        self.syntax_trees.get(&file_id)
    }
}

impl Desugar for Parser {
    type Unsweetened = Unsweetened;

    fn desugar(self) -> Self::Unsweetened {
        Unsweetened {
            files: self.files,
            syntax_trees: self
                .syntax_trees
                .into_iter()
                .map(|(file_id, s)| (file_id, s.desugar()))
                .collect(),
        }
    }
}

impl From<Parser> for Unsweetened {
    fn from(parser: Parser) -> Self {
        parser.desugar()
    }
}

pub trait Desugar: Sized {
    type Unsweetened;

    fn desugar(self) -> Self::Unsweetened;
}

#[cfg(test)]
mod tests {
    use super::{Parser, Desugar};

    #[test]
    fn test_unsweetened() {
        let source = include_str!("../../../../example.gno");

        let mut parser = Parser::new();
        let result = parser.file("example.gno", source);

        let unsweetened = parser.desugar();

        match result {
            Ok(file_id) => println!("{:#?}", unsweetened.syntax_tree(file_id).unwrap()),
            Err(error) => println!("Err: {error}"),
        }
    }
}