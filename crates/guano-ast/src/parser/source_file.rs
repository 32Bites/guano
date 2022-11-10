use owning_ref::RcRef;
use pest::{error::Error, Parser};
use thiserror::Error;

use super::{
    declaration::{Declaration, DeclarationError},
    InternalParser, Rule,
};

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub declarations: Vec<Declaration>,
}

impl SourceFile {
    pub fn parse(input: RcRef<str>) -> Result<Self, SourceFileError> {
        let input_str = input.as_ref();

        let source_file = InternalParser::parse(Rule::source_file, input_str)?
            .next()
            .unwrap();

        let mut declarations = vec![];

        for decl in source_file.into_inner() {
            if let Rule::EOI = decl.as_rule() {
                continue;
            }
            declarations.push(Declaration::parse(decl, input.clone())?);
        }
        
        Ok(
            SourceFile { declarations }
        )
    }
}

#[derive(Debug, Clone, Error)]
pub enum SourceFileError {
    #[error("pest {0}")]
    PestError(#[from] Error<Rule>),
    #[error("{0}")]
    DeclarationError(#[from] DeclarationError),
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::SourceFile;

    #[test]
    fn test_source_file() {
        let source = include_str!("../../../../example.gno");
        let rc_source: Rc<str> = source.into();

        let source_file = SourceFile::parse(rc_source.into());

        match source_file {
            Ok(s) => println!("{s:#?}"),
            Err(error) => println!("Err: {error}"),
        }
    }
}
