use owning_ref::RcRef;
use pest::{iterators::Pair};
use thiserror::Error;

use crate::parser::{typing::Type, Rule, span::{SpanStr, Span, IntoSpan}};

use super::{
    function::{FunctionDeclaration, FunctionError},
    prototype::parse_prototypes,
};

#[derive(Debug, Clone)]
pub struct ClassDeclaration {
    pub name: SpanStr,
    pub super_class: Option<SpanStr>,
    pub prototypes: Vec<SpanStr>,
    pub items: Vec<ClassItem>,
    pub span: Span,
}

impl ClassDeclaration {
    pub fn parse(pair: Pair<'_, Rule>, input: RcRef<str>) -> Result<Self, ClassError> {
        let span = pair.as_span().into_span(input.clone());
        let mut inner = pair.into_inner();
        let name = inner.next().unwrap().into_span_str(input.clone());

        let super_class = match inner.peek() {
            Some(next) => {
                if let Rule::identifier = next.as_rule() {
                    inner.next();
                    Some(next.into_span_str(input.clone()))
                } else {
                    None
                }
            }
            _ => None,
        };

        let prototypes = match inner.peek() {
            Some(next) => {
                if let Rule::prototypes = next.as_rule() {
                    inner.next();
                    parse_prototypes(next, input.clone())
                } else {
                    vec![]
                }
            }
            _ => vec![],
        };

        let mut items = vec![];

        for item in inner {
            items.push(ClassItem::parse(item, input.clone())?);
        }

        Ok(ClassDeclaration {
            name,
            super_class,
            prototypes,
            items,
            span,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ClassItem {
    pub name: SpanStr,
    pub modifier: ClassItemModifier,
    pub kind: ClassItemKind,
    pub span: Span,
}

impl ClassItem {
    pub fn parse(pair: Pair<'_, Rule>, input: RcRef<str>) -> Result<Self, ClassError> {
        let span = pair.as_span().into_span(input.clone());
        let (modifier_removed, modifier) = ClassItemModifier::from_str(pair.as_str());

        let (name, kind) = match pair.as_rule() {
            Rule::class_property => {
                let is_redeclarable = modifier_removed.starts_with("var");

                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().into_span_str(input.clone());
                let property_type = Type::parse(inner.next().unwrap(), input);

                (
                    name,
                    ClassItemKind::Property {
                        is_redeclarable,
                        property_type,
                    },
                )
            }
            Rule::class_method => {
                let function = FunctionDeclaration::parse(pair.into_inner().next().unwrap(), input.clone())?;
                (function.name().clone(), ClassItemKind::Method(function))
            }
            _ => unreachable!(),
        };

        Ok(ClassItem {
            name,
            modifier,
            kind,
            span,
        })
    }
}

#[derive(Debug, Clone)]
pub enum ClassItemKind {
    Property {
        is_redeclarable: bool,
        property_type: Type,
    },
    Method(FunctionDeclaration),
}

#[derive(Debug, Clone)]
pub enum ClassItemModifier {
    Static,
    Private,
    None,
}

impl ClassItemModifier {
    fn from_str(mut string: &str) -> (&str, ClassItemModifier) {
        let len = string.len();

        string = string.trim_start_matches("static");
        if string.len() != len {
            return (string, ClassItemModifier::Static);
        }

        string = string.trim_start_matches("priv");
        if string.len() != len {
            return (string, ClassItemModifier::Private);
        }

        (string, ClassItemModifier::None)
    }
}

#[derive(Debug, Clone, Error)]
pub enum ClassError {
    #[error("{0}")]
    FunctionError(#[from] FunctionError),
}
