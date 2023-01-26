use crate::ast::{declaration::function::Func, prelude::*};

#[derive(Debug, Clone)]
pub struct ProtoImpl {
    prototype: Path,
    ty: Type,
    methods: Vec<Func>,
    span: NodeSpan,
}

impl ProtoImpl {
    pub fn prototype(&self) -> &Path {
        &self.prototype
    }

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
