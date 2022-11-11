use crate::{
    desugar::Desugar,
    parser::{
        declaration::class::{
            ClassDeclaration as ParseClassDeclaration, ClassItem as ParseClassItem,
            ClassItemKind as ParseClassItemKind, ClassItemModifier,
        },
        span::{Span, SpanStr},
        typing::Type,
    },
};

use super::function::FunctionDeclaration;

#[derive(Debug, Clone)]
pub struct ClassDeclaration {
    pub name: SpanStr,
    pub super_class: Option<SpanStr>,
    pub prototypes: Vec<SpanStr>,
    pub items: Vec<ClassItem>,
    pub span: Span,
}

impl Desugar for ParseClassDeclaration {
    type Unsweetened = ClassDeclaration;

    fn desugar(self) -> Self::Unsweetened {
        ClassDeclaration {
            name: self.name,
            super_class: self.super_class,
            prototypes: self.prototypes,
            items: self.items.into_iter().map(|c| c.desugar()).collect(),
            span: self.span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClassItem {
    pub name: SpanStr,
    pub modifier: ClassItemModifier,
    pub kind: ClassItemKind,
    pub span: Span,
}

impl Desugar for ParseClassItem {
    type Unsweetened = ClassItem;

    fn desugar(self) -> Self::Unsweetened {
        ClassItem {
            name: self.name,
            modifier: self.modifier,
            kind: self.kind.desugar(),
            span: self.span,
        }
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

impl Desugar for ParseClassItemKind {
    type Unsweetened = ClassItemKind;

    fn desugar(self) -> Self::Unsweetened {
        match self {
            ParseClassItemKind::Property {
                is_redeclarable,
                property_type,
            } => ClassItemKind::Property {
                is_redeclarable,
                property_type,
            },
            ParseClassItemKind::Method(m) => ClassItemKind::Method(m.desugar()),
        }
    }
}
