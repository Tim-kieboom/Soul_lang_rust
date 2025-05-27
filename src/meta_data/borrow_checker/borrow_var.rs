use std::collections::{BTreeMap, HashMap};
use crate::meta_data::scope_and_var::scope::ScopeId;

use super::borrow_checker::BorrowResult;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct VarId(pub u32);
impl VarId {
    pub fn increment(&mut self) {
        self.0 += 1
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BorrowVar {
    pub parent: Option<VarId>,

    pub name: String,
    pub valid: bool,
    pub mut_ref: Option<VarId>,
    pub refs: HashMap<String, VarId>,
}

impl BorrowVar {
    pub fn new(name: String, parent: Option<VarId>) -> Self {
        BorrowVar{ 
            parent,
            name, 
            valid: true,
            mut_ref: None, 
            refs: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct VarIdStore{ store: BTreeMap<String, VarId> }
impl VarIdStore {
    pub fn new() -> Self {
        Self { store: BTreeMap::new() }
    }

    pub fn get(&self, key: &str) -> Option<&VarId> {
        self.store.get(key)
    }

    pub fn consume_store(self) -> BTreeMap<String, VarId> {
        self.store
    }
}

#[derive(Debug)]
pub struct BorrowVarStore {
    vars: HashMap<VarId, BorrowVar>,
    scope_store: HashMap<ScopeId, VarIdStore>,
    next_var_id: VarId
}

impl BorrowVarStore {
    pub fn new() -> Self {
        BorrowVarStore { 
            vars: HashMap::new(),
            scope_store: HashMap::new(),
            next_var_id: VarId(0)
        }
    }

    pub fn add_scope(&mut self, id: &ScopeId) -> BorrowResult<()> {
        if self.scope_store.contains_key(id) {
            return Err(format!("Internal error: borrow_store already has scope: '{}'", id.0));
        }

        self.scope_store.insert(*id, VarIdStore::new());
        Ok(())
    }

    pub fn add_variable(&mut self, id: &ScopeId, borrow_var: BorrowVar) -> BorrowResult<VarId> {
        let scope = self.scope_store.get_mut(id)
            .ok_or(format!("Internal Error: coould not find scope_id: {}", id.0))?;

        let next_id = self.next_var_id;
        self.next_var_id.increment();

        scope.store.insert(borrow_var.name.clone(), next_id);
        self.vars.insert(next_id, borrow_var);
        Ok(next_id)
    }
    
    pub fn get_scope(&self, id: &ScopeId) -> Option<&VarIdStore> {
        self.scope_store.get(id)
    }

    pub fn get_var(&self, id: &VarId) -> Option<&BorrowVar> {
        self.vars.get(id)
    }

    pub fn get_var_mut(&mut self, id: &VarId) -> Option<&mut BorrowVar> {
        self.vars.get_mut(id)
    }

    pub fn consume_scope(&mut self, id: &ScopeId) -> BorrowResult<VarIdStore> {
        self.scope_store.remove(id)
            .ok_or(format!("Internal Error: tryed to remove scope: '{}' but scope not found", id.0))
    }
    
    pub fn remove_var(&mut self, id: &VarId) -> BorrowResult<()> {
        self.vars.remove(id)
            .ok_or(format!("Internal Error: tryed to remove var: '{}' but var not found", id.0))?;

        Ok(())
    }
}







