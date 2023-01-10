use crate::ast::prelude::*;

use self::{
    class::Class,
    function::{Func, FuncBlock},
    import::Import,
    modifier::Modifier,
    module::Module,
    prototype::Proto,
    variable::Var,
};

pub mod class;
pub mod function;
pub mod import;
pub mod modifier;
pub mod module;
pub mod prototype;
pub mod variable;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Decl {
    kind: DeclKind,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
}

impl Decl {
    pub fn kind(&self) -> &DeclKind {
        &self.kind
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(input: Span) -> Res<Self> {
        let (input, (span, kind)) = consumed(alt((
            map(Class::parse, DeclKind::Class),
            map(
                Func::parse(FuncBlock::Needed, Modifier::Pub),
                DeclKind::Func,
            ),
            map(Import::parse, DeclKind::Import),
            map(Proto::parse, DeclKind::Proto),
            map(Var::parse(Modifier::Pub), DeclKind::Var),
            map(Module::parse, DeclKind::Module),
        )))(input)?;

        let decl = Self {
            kind,
            span: span.into_node(),
        };

        Ok((input, decl))
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum DeclKind {
    Class(Class),
    Func(Func),
    Import(Import),
    Proto(Proto),
    Var(Var),
    Module(Module),
}

impl Default for DeclKind {
    fn default() -> Self {
        Self::Import(Import::default())
    }
}
