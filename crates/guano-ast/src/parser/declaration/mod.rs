use owning_ref::RcRef;
use pest::iterators::Pair;
use thiserror::Error;

use self::{
    class::{ClassDeclaration, ClassError},
    function::{FunctionDeclaration, FunctionError},
    variable::{VariableDeclaration, VariableError}, prototype::{PrototypeDeclaration, PrototypeError}, type_alias::TypeAliasDeclaration, import::ImportDeclaration,
};
use super::{Rule, literal::EscapeError};

pub mod class;
pub mod function;
pub mod import;
pub mod procedure;
pub mod prototype;
pub mod type_alias;
pub mod variable;

#[derive(Debug, Clone)]
pub enum Declaration {
    Variable(VariableDeclaration),
    Function(FunctionDeclaration),
    Class(ClassDeclaration),
    Prototype(PrototypeDeclaration),
    TypeAlias(TypeAliasDeclaration),
    Import(ImportDeclaration)
}

impl Declaration {
    pub fn parse(pair: Pair<'_, Rule>, input: RcRef<str>) -> Result<Self, DeclarationError> {
        Ok(match pair.as_rule() {
            Rule::declaration => return Self::parse(pair.into_inner().next().unwrap(), input),
            Rule::variable_declaration => Declaration::Variable(VariableDeclaration::parse(pair, input)?),
            Rule::function_declaration => Declaration::Function(FunctionDeclaration::parse(pair, input)?),
            Rule::class_declaration => Declaration::Class(ClassDeclaration::parse(pair, input)?),
            Rule::prototype_declaration => Declaration::Prototype(PrototypeDeclaration::parse(pair, input)?),
            Rule::type_alias_declaration => Declaration::TypeAlias(TypeAliasDeclaration::parse(pair, input)),
            Rule::import_declaration => Declaration::Import(ImportDeclaration::parse(pair, input)?),
            r => panic!("{r:?} {:?}", pair.as_str()),
        })
    }
}

#[derive(Debug, Clone, Error)]
pub enum DeclarationError {
    #[error("{0}")]
    VariableError(#[from] VariableError),
    #[error("{0}")]
    FunctionError(#[from] FunctionError),
    #[error("{0}")]
    ClassError(#[from] ClassError),
    #[error("{0}")]
    PrototypeError(#[from] PrototypeError),
    #[error("{0}")]
    EscapeError(#[from] EscapeError),
}
