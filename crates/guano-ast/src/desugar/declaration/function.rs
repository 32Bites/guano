use indexmap::IndexMap;

use crate::{
    desugar::{block::Block, Desugar},
    parser::{
        declaration::{procedure::{Argument, ProcedureDeclaration}, function::FunctionDeclaration as ParseFunctionDeclaration},
        span::{Span, SpanStr},
        typing::Type,
    },
};

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub procedure: ProcedureDeclaration,
    pub block: Block,
    pub span: Span,
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
}


impl Desugar for ParseFunctionDeclaration {
    type Unsweetened = FunctionDeclaration;

    fn desugar(self) -> Self::Unsweetened {
        FunctionDeclaration {
            procedure: self.procedure,
            span: self.span,
            block: self.block.desugar()
        }
    }
}