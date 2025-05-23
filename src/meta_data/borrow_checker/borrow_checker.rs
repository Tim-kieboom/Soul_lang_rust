use std::collections::HashMap;
use super::borrow_var::{BorrowVarStore, VarId};
use crate::meta_data::{borrow_checker::borrow_var::BorrowVar, scope_and_var::scope::ScopeId};
pub type BorrowResult<T> = std::result::Result<T, String>;

pub type DeleteList = Vec<String>;

pub struct BorrowId<'a>(pub &'a str, pub &'a ScopeId);
pub trait BorrowCheckedTrait {
    /// Registers a new owner (variable) in the borrow checker.
    ///
    /// # Arguments
    /// * `owner` - The identifier for the new owner to track.
    ///
    /// # Errors
    /// Returns an error if the owner is already declared or another invariant is violated.
    fn declare_owner(&mut self, owner: &BorrowId) -> BorrowResult<()>;

    /// Creates an immutable (const) borrow of an owner.
    ///
    /// # Arguments
    /// * `owner` - The variable being borrowed.
    /// * `parent` - The context (usually the scope or variable) that is borrowing.
    ///
    /// # Errors
    /// Returns an error if a mutable borrow exists or other rules are violated.
    fn borrow_const(&mut self, owner: &BorrowId, parent: &BorrowId) -> BorrowResult<()>;

    /// Creates a mutable borrow of an owner.
    ///
    /// # Arguments
    /// * `owner` - The variable being mutably borrowed.
    /// * `parent` - The context (usually the scope or variable) that is borrowing.
    ///
    /// # Errors
    /// Returns an error if any other borrows exist or other rules are violated.
    fn borrow_mut(&mut self, owner: &BorrowId, parent: &BorrowId) -> BorrowResult<()>;

    /// Transfers ownership from one variable to another (move semantics).
    ///
    /// # Arguments
    /// * `old_owner` - The current owner.
    /// * `new_owner` - The new owner.
    ///
    /// # Errors
    /// Returns an error if the move is invalid (e.g., outstanding borrows).
    fn move_owner(&mut self, old_owner: &BorrowId, new_owner: &BorrowId) -> BorrowResult<()>;

    /// Opens a new scope in the borrow checker.
    ///
    /// # Arguments
    /// * `scope_id` - The identifier for the new scope.
    ///
    /// # Errors
    /// Returns an error if the scope cannot be opened (e.g., already open).
    fn open_scope(&mut self, scope_id: &ScopeId) -> BorrowResult<()>;

    /// drop an owner making owner and refs invalid
    ///
    /// # Arguments
    /// * `owner` - owner to drop.
    ///
    /// # Errors
    /// Returns an error if the owner could not be dropped (e.g., not found).
    fn drop_owner(&mut self, owner: &BorrowId) -> BorrowResult<()>;

    /// Closes a scope, returns list of all owners that have to be cleaned up
    ///
    /// # Arguments
    /// * `scope_id` - The identifier for the scope being closed.
    ///
    /// # Returns
    /// A list of variable names that should be deleted or dropped.
    ///
    /// # Errors
    /// Returns an error if the scope cannot be closed (e.g., not found).
    fn close_scope(&mut self, scope_id: &ScopeId) -> BorrowResult<DeleteList>;
} 

pub struct BorrowChecker {
    borrow_store: BorrowVarStore 
}

impl BorrowChecker {
    pub fn new() -> Self {
        BorrowChecker { 
            borrow_store: BorrowVarStore::new() 
        }
    }
}

impl BorrowCheckedTrait for BorrowChecker {
    fn declare_owner(&mut self, owner: &BorrowId) -> BorrowResult<()> {
        let BorrowId(name, scope_id) = *owner;
        
        let scope = self.borrow_store.get_scope(&scope_id)
            .ok_or(format!("Internal Error: could not find scope of scope_id: {}", scope_id.0))?;

        if let Some(declared_owner) = scope.get(name) {
            let valid = self.borrow_store.get_var(declared_owner).expect("Internal error declared_owner not found").valid;
            if valid {
                return Err(format!("scope: '{}' already has borrowchecked var: '{}'", scope_id.0, name));
            }
        }

        self.borrow_store.add_variable(scope_id, BorrowVar::new(name.to_string(), None))?;

        Ok(())
    }

    fn borrow_const(&mut self, borrow: &BorrowId, parent: &BorrowId) -> BorrowResult<()> {
        let BorrowId(borrow_name, borrow_scope_id) = *borrow;
        let BorrowId(parent_name, _) = *parent;

        let parent_id = self.get_var_id(parent)
            .map_err(|msg| format!("in owner: '{}' constref to: '{}', {}", parent_name, borrow_name, msg))?;

        self.borrow_store.get_scope(borrow_scope_id)
            .ok_or(format!("Internal Error: could not find scope of scope_id: {}", borrow_scope_id.0))?;

        let borrow_id = self.borrow_store.add_variable(borrow_scope_id, BorrowVar::new(borrow_name.to_string(), Some(parent_id)))?;
        
        let parent = self.borrow_store.get_var_mut(&parent_id)
            .ok_or(format!("Internal Error: parent: '{}' not found in borrow_store", parent_name))?;

        parent.refs.insert(borrow_name.to_string(), borrow_id);
        
        Ok(())
    }

    fn borrow_mut(&mut self, borrow: &BorrowId, parent: &BorrowId) -> BorrowResult<()> {
        let BorrowId(borrow_name, borrow_scope_id) = *borrow;
        let BorrowId(parent_name, _) = *parent;

        let parent_id = self.get_var_id(parent)
            .map_err(|msg| format!("in owner: '{}' mutref to: '{}', {}", parent_name, borrow_name, msg))?;

        if self.borrow_store.get_var(&parent_id)
            .ok_or(format!("Internal Error: parent: '{}' not found in borrow_store", parent_name))?
            .mut_ref 
            .is_some()
        {
            return Err(format!("in owner: '{}' mutref to '{}', owner: '{}' already has a mutref", parent_name, borrow_name, parent_name));
        }

        self.borrow_store.get_scope(borrow_scope_id)
            .ok_or(format!("Internal Error: could not find scope of scope_id: {}", borrow_scope_id.0))?;

        let borrow_id = self.borrow_store.add_variable(borrow_scope_id, BorrowVar::new(borrow_name.to_string(), Some(parent_id)))?;
        
        let parent = self.borrow_store.get_var_mut(&parent_id).unwrap();
        parent.mut_ref = Some(borrow_id);
        
        Ok(())
    }

    fn move_owner(&mut self, old_owner: &BorrowId, new_owner: &BorrowId) -> BorrowResult<()> {
        let BorrowId(old_name, _) = *old_owner;
        let BorrowId(new_name, _) = *new_owner;

        let pass_err = |msg: String| { 
            format!("in owner: '{}' move to: '{}', {}", old_name, new_name, msg) 
        };

        let new_id = self.get_var_id(new_owner)
            .map_err(|msg| pass_err(msg))?;

        let old_id = self.get_var_id(old_owner)
            .map_err(|msg| pass_err(msg))?;

        let (valid, refs, mut_ref) = self.invalidate_owner(old_id);

        let new_var = self.borrow_store.get_var_mut(&new_id)
            .ok_or(format!("in owner: '{}' move to: '{}', var: '{}' not found", old_name, new_name, new_name))?;

        new_var.valid = valid;
        new_var.refs = refs;
        new_var.mut_ref = mut_ref;

        Ok(())
    }

    fn open_scope(&mut self, scope_id: &ScopeId) -> BorrowResult<()> {
        self.borrow_store.add_scope(scope_id)
    }

    fn drop_owner(&mut self, owner: &BorrowId) -> BorrowResult<()> {
        let BorrowId(name, _) = *owner;

        let var_id = self.get_var_id(owner)
            .map_err(|msg| format!("in drop of: '{}' , {}", name, msg) )?;

        let _ = self.invalidate_owner(var_id);
        Ok(())
    }

    fn close_scope(&mut self, scope_id: &ScopeId) -> BorrowResult<DeleteList> {

        let mut delete_list = DeleteList::new();

        let scope = self.borrow_store.consume_scope(scope_id)?;

        for (name, id) in scope {
            let parent = self.borrow_store.get_var_mut(&id)
                .ok_or(format!("Internal Error: var: '{}' not found", name))?
                .parent.clone();

            let (valid, _, _) = self.invalidate_owner(id);

            if parent.is_none() {
                if valid {
                    delete_list.push(name.clone());
                }
            } 

            self.borrow_store.remove_var(&id)?;
        }

        Ok(delete_list)
    }

}
 
impl BorrowChecker {
    
    fn get_var_id(&self, borrow_var: &BorrowId) -> BorrowResult<VarId> {
        let BorrowId(name, scope_id) = *borrow_var;

        let scope = self.borrow_store.get_scope(scope_id)
            .ok_or(format!("Internal Error: could not find scope of scope_id: {}", scope_id.0))?;
    
        let id = *scope.get(name)
            .ok_or(format!("var: '{}' is not found", name))?;

        let is_valid = self.borrow_store.get_var(&id)
            .ok_or(format!("var: '{}' is not found", name))?.valid;

        if !is_valid {
            return Err(format!("var: '{}' is not valid", name));
        }

        Ok(id)
    }

    fn invalidate_owner(&mut self, owner_id: VarId) -> (bool, HashMap<String, VarId>, Option<VarId>) {        
        let (valid, parent_ref, refs, mut mut_ref) = {
            
            let owner = self.borrow_store.get_var_mut(&owner_id)
                .expect("Internal error owner not found");
            
            let valid = owner.valid;
            owner.valid = false;
            let refs = owner.refs.drain().collect::<HashMap<_, _>>();
            let mut_ref = owner.mut_ref.clone();
            let parent = owner.parent.clone();
            (valid, parent, refs, mut_ref)
        };

        if let Some(parent_id) = parent_ref {
            self.deleting_child(parent_id, owner_id);            
        }

        for (_, var_id) in &refs {
            let const_ref = self.borrow_store.get_var_mut(var_id)
                .expect("Internal error on owner.refs not found");

            const_ref.valid = false;
        }

        if let Some(ref_id) = &mut mut_ref {
            let mut_ref = self.borrow_store.get_var_mut(&ref_id)
                .expect("Internal error owner.mut_ref not found");

            mut_ref.valid = false;
        }

        (valid, refs, mut_ref)
    }

    fn deleting_child(&mut self, parent_id: VarId, child_id: VarId) {
        let child_name = self.borrow_store.get_var(&child_id)
            .expect("Internal error owner not found")
            .name.clone();
        
        let parent = self.borrow_store.get_var_mut(&parent_id)
            .expect("Internal error owner not found");
        
        if let Some(mut_ref) = &parent.mut_ref {
            if mut_ref == &child_id {
                parent.mut_ref = None;
                return;
            }
        }

        parent.refs.remove(&child_name);
    }
}




















