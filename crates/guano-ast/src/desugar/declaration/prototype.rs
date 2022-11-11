use crate::{
    desugar::{block::Block, Desugar},
    parser::{declaration::{
        procedure::ProcedureDeclaration, prototype::{PrototypeItemKind as ParsePrototypeItemKind, PrototypeItem as ParsePrototypeItem, PrototypeDeclaration as ParsePrototypeDeclaration},
    }, span::{SpanStr, Span}},
};

#[derive(Debug, Clone)]
pub struct PrototypeDeclaration {
    pub name: SpanStr,
    pub parent_prototypes: Vec<SpanStr>,
    pub items: Vec<PrototypeItem>,
    pub span: Span
}

impl Desugar for ParsePrototypeDeclaration {
    type Unsweetened = PrototypeDeclaration;

    fn desugar(self) -> Self::Unsweetened {
        PrototypeDeclaration {
            name: self.name,
            parent_prototypes: self.parent_prototypes,
            items: self.items.into_iter().map(|i| i.desugar()).collect(),
            span: self.span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrototypeItem {
    pub name: SpanStr,
    pub is_static: bool,
    pub kind: PrototypeItemKind,
    pub span: Span
}

impl Desugar for ParsePrototypeItem {
    type Unsweetened = PrototypeItem;

    fn desugar(self) -> Self::Unsweetened {
        PrototypeItem {
            name: self.name,
            is_static: self.is_static,
            kind: self.kind.desugar(),
            span: self.span,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PrototypeItemKind {
    Method {
        procedure: ProcedureDeclaration,
        block: Option<Block>,
    },
}

impl Desugar for ParsePrototypeItemKind {
    type Unsweetened = PrototypeItemKind;

    fn desugar(self) -> Self::Unsweetened {
        match self {
            ParsePrototypeItemKind::Method { procedure, block } => PrototypeItemKind::Method {
                procedure,
                block: block.map(|b| b.desugar()),
            },
        }
    }
}
