use indexmap::IndexMap;

use super::typing::Type;

pub type ScopeId = usize;

pub enum ScopeCategory {
    Global,
    Function,
    Block // Expression?
}

pub struct Scope {
    pub id: ScopeId,
    pub parent: Option<ScopeId>,
    pub category: ScopeCategory,
    pub values: IndexMap<String, Type>,
    pub children: Vec<ScopeId>
}