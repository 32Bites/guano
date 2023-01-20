use self::{prototype::ProtoImpl, ty::TypeImpl};

use super::prelude::*;

pub mod prototype;
mod r#type;

pub mod ty {
    pub use super::r#type::*;
}

#[derive(Debug, Clone)]
pub struct Impl {
    kind: ImplKind,
    span: NodeSpan,
}

impl Impl {
    pub fn kind(&self) -> &ImplKind {
        &self.kind
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(input: Span) -> Res<Self> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum ImplKind {
    Proto(ProtoImpl),
    Type(TypeImpl),
}
