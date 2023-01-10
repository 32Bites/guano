use owning_ref::RcRef;
use pest::{
    iterators::Pair,
};
use thiserror::Error;

use crate::parser::{
    expression::{Expression, ExpressionError},
    typing::Type,
    Rule,
    span::{Span, IntoSpan, SpanStr}
};

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub is_redeclarable: bool,
    pub name: SpanStr,
    pub variable_type: Option<Type>,
    pub variable_value: Option<Expression>,
    pub span: Span,
}

impl VariableDeclaration {
    pub fn parse(pair: Pair<'_, Rule>, input: RcRef<str>) -> Result<Self, VariableError> {
        if let Rule::declaration = pair.as_rule() {
            return Self::parse(pair.into_inner().next().unwrap(), input)
        }

        let span = pair.as_span().into_span(input.clone());
        let mut inner = pair.into_inner();
        let is_redeclarable = match inner.next().unwrap().as_str() {
            "var" => true,
            _ => false
        };

        let name = inner.next().unwrap().into_span_str(input.clone());

        let mut next = inner.next();

        let variable_type = match next.take() {
            Some(next_) => {
                if let Rule::declaration_type = next_.as_rule() {
                    next = inner.next();
                    let ty = Type::parse(next_.into_inner().next().unwrap(), input.clone());

                    Some(ty)
                } else {
                    next = Some(next_);
                    None
                }
            }
            _ => unreachable!(),
        };

        let variable_value = match next.take() {
            Some(next) => {
                let expression = next.into_inner().next().unwrap();
                Some(Expression::parse(expression.into_inner(), input)?)
            }
            None => None,
        };

        Ok(VariableDeclaration {
            is_redeclarable,
            name,
            variable_type,
            variable_value,
            span,
        })
    }
}

#[derive(Debug, Clone, Error)]
pub enum VariableError {
    #[error("{0}")]
    ExpressionError(#[from] ExpressionError),
}
