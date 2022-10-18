use indexmap::IndexMap;
use thiserror::Error;

use crate::parser::typing::Type;

pub struct Function {
    pub name: String,
    pub return_type: Option<Type>,
    pub arguments: IndexMap<String, Type>,
}

#[derive(Debug, Error)]
pub enum FunctionError {}
