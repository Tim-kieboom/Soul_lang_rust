use std::collections::{BTreeMap, HashMap};

use super::var_info::VarInfo;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ScopeId(pub u64);
impl ScopeId {
    pub fn increment(&self) -> ScopeId {
        ScopeId(self.0 + 1)
    }
}

pub struct Scope {
    id: ScopeId,
    // borrow_checker: &'a BorrowChecker,

    pub parent: Option<ScopeId>,
    last_child_id: ScopeId,
    pub vars: BTreeMap<String, VarInfo>,
} 

impl Scope {
    pub fn new_global() -> Self {
        Scope { 
            id: ScopeId(0), 
            parent: None, 
            last_child_id: ScopeId(0),
            vars: BTreeMap::new(), 
        }
    }

    pub fn new_child(parent: &Scope) -> Self {
        let child_id = parent.last_child_id.increment();
        Scope { 
            id: child_id, 
            parent: Some(parent.id), 
            last_child_id: child_id,
            vars: BTreeMap::new(), 
        }
    }

    pub fn id(&self) -> &ScopeId {
        &self.id
    }

    pub fn try_get_variable<'a>(&'a self, var_name: &String, scopes: &'a HashMap<ScopeId, Scope>) -> Option<&'a VarInfo> {
        if let Some(var) = self.vars.get(var_name) {
            return Some(var);
        }

        let mut current_scope = self;
        while let Some(parent) = current_scope.parent {
            current_scope = scopes.get(&parent)?;
            
            if let Some(var) = current_scope.vars.get(var_name) {
                return Some(var);
            }
        }

        None
    }
}