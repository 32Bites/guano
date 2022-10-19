use guano_lexer::{Span, Token};
use thiserror::Error;

use crate::{
    convert_result_impl,
    parser::{ConvertResult, Parse, Parser},
};

use super::{
    function::FunctionError,
    variable::{Variable, VariableError},
};

#[derive(Debug, Clone)]
pub enum Statement {
    Variable(Variable),
}

impl<I: Iterator<Item = (Token, Span)> + std::fmt::Debug> Parse<I, StatementError> for Statement {
    fn parse(parser: &mut Parser<I>) -> Result<Statement, Option<StatementError>> {
        Ok(Statement::Variable(
            Variable::parse(parser).convert_result()?,
        ))
    }
}

#[derive(Debug, Error)]
pub enum StatementError {
    #[error("{0}")]
    InvalidVariable(#[from] VariableError),
    #[error("{0}")]
    InvalidFunction(#[from] FunctionError),
}

convert_result_impl!(StatementError);
