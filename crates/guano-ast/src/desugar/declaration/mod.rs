use crate::parser::declaration::{
    import::ImportDeclaration, type_alias::TypeAliasDeclaration, variable::VariableDeclaration,
    Declaration as ParseDeclaration,
};

use self::{
    class::ClassDeclaration, function::FunctionDeclaration, prototype::PrototypeDeclaration,
};

use super::Desugar;

pub mod class;
pub mod function;
pub mod prototype;

#[derive(Debug, Clone)]
pub enum Declaration {
    Variable(VariableDeclaration),
    TypeAlias(TypeAliasDeclaration),
    Import(ImportDeclaration),
    Class(ClassDeclaration),
    Function(FunctionDeclaration),
    Prototype(PrototypeDeclaration),
}

impl Desugar for ParseDeclaration {
    type Unsweetened = Declaration;

    fn desugar(self) -> Self::Unsweetened {
        match self {
            ParseDeclaration::Function(f) => Declaration::Function(f.desugar()),
            ParseDeclaration::Class(c) => Declaration::Class(c.desugar()),
            ParseDeclaration::Prototype(p) => Declaration::Prototype(p.desugar()),
            ParseDeclaration::Variable(v) => Declaration::Variable(v),
            ParseDeclaration::TypeAlias(t) => Declaration::TypeAlias(t),
            ParseDeclaration::Import(i) => Declaration::Import(i),
        }
    }
}
