use self::{prototype::ProtoImpl, ty::TypeImpl};

use super::prelude::*;

pub mod prototype;
mod r#type;

pub mod ty {
    pub use super::r#type::*;
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Impl {
    kind: ImplKind,
    #[cfg_attr(feature = "serde", serde(skip))]
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
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ImplKind {
    Proto(ProtoImpl),
    Type(TypeImpl),
}
