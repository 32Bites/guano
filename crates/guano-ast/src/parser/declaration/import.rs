use owning_ref::RcRef;
use pest::{iterators::Pair};

use crate::parser::{
    literal::{parse_string_literal, EscapeError},
    Rule, span::{SpanStr, Span, IntoSpan},
};

#[derive(Debug, Clone)]
pub struct ImportDeclaration {
    pub import: String,
    pub alias: Option<SpanStr>,
    pub span: Span,
}

impl ImportDeclaration {
    pub fn parse(pair: Pair<'_, Rule>, input: RcRef<str>) -> Result<Self, EscapeError> {
        let span = pair.as_span().into_span(input.clone());
        let mut inner = pair.into_inner();

        let import = parse_string_literal(inner.next().unwrap())?;
        let alias = inner.next().map(|a| a.into_span_str(input));

        Ok(ImportDeclaration {
            import,
            alias,
            span,
        })
    }
}
