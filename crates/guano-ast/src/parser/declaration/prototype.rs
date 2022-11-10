use crate::parser::{block::Block, statement::StatementError, span::{SpanStr, Span, IntoSpan}};
use owning_ref::RcRef;
use pest::{iterators::Pair};
use thiserror::Error;

use super::{
    procedure::{ProcedureDeclaration, ProcedureError},
    Rule,
};

#[derive(Debug, Clone)]
pub struct PrototypeDeclaration {
    pub name: SpanStr,
    pub parent_prototypes: Vec<SpanStr>,
    pub items: Vec<PrototypeItem>,
    pub span: Span,
}

impl PrototypeDeclaration {
    pub fn parse(pair: Pair<'_, Rule>, input: RcRef<str>) -> Result<Self, PrototypeError> {
        let span = pair.as_span().into_span(input.clone());
        let mut inner = pair.into_inner();

        let name = inner.next().unwrap().into_span_str(input.clone());

        let parent_prototypes = match inner.peek() {
            Some(maybe_parent_prototypes) => {
                if let Rule::prototypes = maybe_parent_prototypes.as_rule() {
                    inner.next();
                    parse_prototypes(maybe_parent_prototypes, input.clone())
                } else {
                    vec![]
                }
            }
            None => vec![],
        };

        let mut items = vec![];

        for item in inner {
            items.push(PrototypeItem::parse(item, input.clone())?);
        }

        Ok(PrototypeDeclaration {
            name,
            parent_prototypes,
            items,
            span,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PrototypeItem {
    pub name: SpanStr,
    pub is_static: bool,
    pub kind: PrototypeItemKind,
    pub span: Span,
}

impl PrototypeItem {
    pub fn parse(pair: Pair<'_, Rule>, input: RcRef<str>) -> Result<Self, PrototypeError> {
        let is_static = pair.as_str().starts_with("static");
        let span = pair.as_span().into_span(input.clone());

        let (name, kind) = match pair.as_rule() {
            Rule::prototype_method => {
                let mut inner = pair.into_inner();
                let procedure = ProcedureDeclaration::parse(inner.next().unwrap(), input.clone())?;
                let block = match inner.next() {
                    Some(block) => Some(Block::parse(block, input)?),
                    None => None,
                };

                (
                    procedure.name.clone(),
                    PrototypeItemKind::Method { procedure, block },
                )
            }
            _ => unreachable!(),
        };

        Ok(PrototypeItem {
            name,
            is_static,
            kind,
            span,
        })
    }
}

#[derive(Debug, Clone)]
pub enum PrototypeItemKind {
    Method {
        procedure: ProcedureDeclaration,
        block: Option<Block>,
    },
}

pub fn parse_prototypes(pair: Pair<'_, Rule>, input: RcRef<str>) -> Vec<SpanStr> {
    let mut prototypes = vec![];

    for prototype in pair.into_inner() {
        prototypes.push(prototype.into_span_str(input.clone()));
    }

    prototypes
}

#[derive(Debug, Clone, Error)]
pub enum PrototypeError {
    #[error("{0}")]
    ProcedureError(#[from] ProcedureError),
    #[error("{0}")]
    StatementError(#[from] StatementError),
}
