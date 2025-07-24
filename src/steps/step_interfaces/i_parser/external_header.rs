use std::collections::HashMap;
use crate::steps::step_interfaces::i_parser::{abstract_syntax_tree::soul_type::type_kind::TypeKind, scope::{ScopeKind}};


#[derive(Debug, Clone)]
pub struct Header {
    pub scope: Vec<ScopeKind>,
    pub types: Vec<TypeKind>,
}

#[derive(Debug, Clone)]
pub struct ExternalHeader {
    pub store: HashMap<String, Header>, 
}

impl Header {
    pub fn from<const N: usize, const M: usize>(scope: [ScopeKind; N], types: [TypeKind; M]) -> Self {
        Self{scope: Vec::from(scope), types: Vec::from(types)}
    }
}

impl ExternalHeader {
    pub fn new() -> Self {
        Self{store: HashMap::new()}
    }

    pub fn from<const N: usize>(arr: [(String, Header); N]) -> Self {
        Self{store: HashMap::from(arr)}
    }
}























