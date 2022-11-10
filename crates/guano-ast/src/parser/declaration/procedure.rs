use indexmap::IndexMap;
use owning_ref::RcRef;
use pest::iterators::Pair;
use thiserror::Error;

use crate::parser::{typing::Type, Rule, span::{Span, SpanStr, IntoSpan}};

#[derive(Debug, Clone)]
pub struct ProcedureDeclaration {
    pub name: SpanStr,
    pub procedure_type: Option<Type>,
    pub procedure_arguments: IndexMap<SpanStr, Argument>,
}

impl ProcedureDeclaration {
    pub fn parse(pair: Pair<'_, Rule>, input: RcRef<str>) -> Result<Self, ProcedureError> {
        let mut inner = pair.into_inner();
        let name = inner.next().unwrap().into_span_str(input.clone());
        let procedure_type = match inner.peek() {
            Some(maybe_type) => {
                if let Rule::declaration_type = maybe_type.as_rule() {
                    Some(Type::parse(inner.next().unwrap(), input.clone()))
                } else {
                    None
                }
            }
            None => None,
        };

        let mut arguments = IndexMap::new();

        if let Some(mut defined_arguments) = inner.next().map(|d| d.into_inner()) {
            while let Some(arg_name) = defined_arguments.next() {
                let ty = Type::parse(defined_arguments.next().unwrap(), input.clone());
                let name = arg_name.as_span().into_span_str(input.clone());
                let span = &arg_name.as_span().into_span(input.clone()) + &ty.span;

                let argument = Argument {
                    name: name.clone(),
                    argument_type: ty,
                    span,
                };

                if let Some(_) = arguments.insert(name.clone(), argument) {
                    panic!("{name} is already defined")
                }
            }
        }

        Ok(ProcedureDeclaration {
            name,
            procedure_type,
            procedure_arguments: arguments,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub name: SpanStr,
    pub argument_type: Type,
    pub span: Span,
}

#[derive(Debug, Clone, Error)]
pub enum ProcedureError {}
