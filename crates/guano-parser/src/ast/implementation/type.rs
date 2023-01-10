use crate::ast::{declaration::function::Func, prelude::*};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TypeImpl {
    ty: Type,
    methods: Vec<Func>,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
}

impl TypeImpl {
    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn methods(&self) -> &[Func] {
        &self.methods
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }
}
