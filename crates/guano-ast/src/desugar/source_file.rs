use crate::parser::source_file::SourceFile as ParseSourceFile;

use super::{declaration::Declaration, Desugar};

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub declarations: Vec<Declaration>,
}

impl Desugar for ParseSourceFile {
    type Unsweetened = SourceFile;

    fn desugar(self) -> Self::Unsweetened {
        SourceFile {
            declarations: self.declarations.into_iter().map(|d| d.desugar()).collect(),
        }
    }
}
