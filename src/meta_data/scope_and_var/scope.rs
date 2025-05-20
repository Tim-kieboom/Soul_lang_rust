use std::{collections::{BTreeMap, HashMap}, sync::{Arc, Mutex}};
use crate::meta_data::borrow_checker::borrow_checker::BorrowChecker;

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
    borrow_checker: Arc<Mutex<BorrowChecker>>,

    pub parent: Option<ScopeId>,
    last_child_id: ScopeId,
    pub vars: BTreeMap<String, VarInfo>,
} 

impl Scope {
    pub fn new_global(borrow_checker: Arc<Mutex<BorrowChecker>>) -> Self {
        Scope { 
            borrow_checker,
            id: ScopeId(0), 
            parent: None, 
            last_child_id: ScopeId(0),
            vars: BTreeMap::new(), 
        }
    }

    pub fn new_child(borrow_checker: Arc<Mutex<BorrowChecker>>, parent: &Scope) -> Self {
        let child_id = parent.last_child_id.increment();
        Scope { 
            borrow_checker,
            id: child_id, 
            parent: Some(parent.id), 
            last_child_id: child_id,
            vars: BTreeMap::new(), 
        }
    }

    pub fn id(&self) -> &ScopeId {
        &self.id
    }

    pub fn try_get_variable<'b>(&'b self, var_name: &String, scopes: &'b HashMap<ScopeId, Scope>) -> Option<&'b VarInfo> {
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















