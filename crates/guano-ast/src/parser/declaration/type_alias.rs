use owning_ref::RcRef;
use pest::iterators::Pair;

use crate::parser::{typing::Type, Rule, span::{SpanStr, Span, IntoSpan}};

#[derive(Debug, Clone)]
pub struct TypeAliasDeclaration {
    pub alias: SpanStr,
    pub aliased_type: Type,
    pub span: Span
}

impl TypeAliasDeclaration {
    pub fn parse(pair: Pair<'_, Rule>, input: RcRef<str>) -> Self {
        let span = pair.as_span().into_span(input.clone());
        let mut inner = pair.into_inner();

        let alias = inner.next().unwrap().into_span_str(input.clone());
        let aliased_type = Type::parse(inner.next().unwrap(), input);

        TypeAliasDeclaration { alias, aliased_type, span }
    }
}