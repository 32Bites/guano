use std::{collections::HashMap, ops::Range};

use guano_lexer::Token;
use thiserror::Error;

use super::{
    error::ParseResult, function::Function, identifier::Identifier,
    statement::variable::VariableDeclaration, Parse, ParseContext, TokenStream,
};

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub function_declarations: HashMap<Identifier, Function>,
    pub global_variables: HashMap<Identifier, VariableDeclaration>,
}

impl Parse<SourceFileError> for SourceFile {
    fn parse(
        context: &mut ParseContext,
    ) -> ParseResult<Self, SourceFileError> {
        todo!()
    }
}

#[derive(Error, Debug)]
pub enum SourceFileError {}
