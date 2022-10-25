use std::collections::HashMap;
use thiserror::Error;

use super::{
    error::ParseResult, function::Function, identifier::Identifier, statement::variable::Variable,
    Parse, ParseContext,
};

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub function_declarations: HashMap<Identifier, Function>,
    pub global_variables: HashMap<Identifier, Variable>,
}

impl Parse<SourceFileError> for SourceFile {
    fn parse(_context: &mut ParseContext) -> ParseResult<Self, SourceFileError> {
        todo!()
    }
}

#[derive(Error, Debug)]
pub enum SourceFileError {}
