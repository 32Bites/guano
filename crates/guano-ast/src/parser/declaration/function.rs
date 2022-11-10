use indexmap::IndexMap;
use owning_ref::RcRef;
use pest::{iterators::Pair};
use thiserror::Error;

use crate::parser::{Rule, block::Block, statement::StatementError, typing::Type, span::{SpanStr, Span, IntoSpan}};

use super::procedure::{ProcedureDeclaration, ProcedureError, Argument};

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub procedure: ProcedureDeclaration,
    pub block: Block,
    pub span: Span
}

impl FunctionDeclaration {
    pub fn name(&self) -> &SpanStr {
        &self.procedure.name
    }

    pub fn return_type(&self) -> Option<&Type> {
        self.procedure.procedure_type.as_ref()
    }

    pub fn parameters(&self) -> &IndexMap<SpanStr, Argument> {
        &self.procedure.procedure_arguments
    }

    pub fn parse(pair: Pair<'_, Rule>, input: RcRef<str>) -> Result<Self, FunctionError> {
        let span = pair.as_span().into_span(input.clone());
        let mut inner = pair.into_inner();

        let procedure = ProcedureDeclaration::parse(inner.next().unwrap(), input.clone())?;
        let block = Block::parse(inner.next().unwrap(), input)?;

        Ok(
            FunctionDeclaration {
                procedure,
                block,
                span,
            }
        )
    }
}

#[derive(Debug, Clone, Error)]
pub enum FunctionError {
    #[error("{0}")]
    ProcedureError(#[from] ProcedureError),
    #[error{"{0}"}]
    StatementError(#[from] StatementError)
}