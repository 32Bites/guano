use crate::ast::{implementation::Impl, prelude::*};

use super::modifier::{Modifier, Modifiers};

#[derive(Debug, Clone)]
pub struct Module {
    modifiers: Modifiers,
    iden: Iden,
    items: Vec<ModuleItem>,
    span: NodeSpan,
}

impl Module {
    pub fn modifiers(&self) -> &Modifiers {
        &self.modifiers
    }

    pub fn iden(&self) -> &Iden {
        &self.iden
    }

    pub fn items(&self) -> &[ModuleItem] {
        &self.items
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(input: Span) -> Res<Self> {
        let (input, modifiers) =
            preceded(Keyword::Module, padded(Modifiers::parse(Modifier::Pub)))(input)?;
        let (input, iden) = padded(Iden::parse)(input)?;
        todo!()
    }
}

pub fn parse_module_items(input: Span) -> Res<Vec<ModuleItem>> {
    todo!()
}

#[derive(Debug, Clone)]
pub enum ModuleItem {
    Decl(Decl),
    Impl(Impl),
}
