use crate::meta_data::soul_error::soul_error::Result;
use std::collections::{BTreeMap, HashMap};
use crate::meta_data::current_context::current_context::{CurrentContext, DefinedGenric};
use crate::meta_data::function::argument_info::argument_info::ArgumentInfo;
use crate::meta_data::meta_data::MetaData;
use crate::meta_data::{function::function_declaration::function_declaration::FunctionID, meta_data::FunctionStore};
use crate::tokenizer::token::TokenIterator;

use super::var_info::VarInfo;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ScopeId(pub u64);
impl ScopeId {
    pub fn increment(&self) -> ScopeId {
        ScopeId(self.0 + 1)
    }
}

pub struct ScopeParentInfo {
    pub id: ScopeId,
    pub allows_vars_access: bool
}
pub struct Scope {
    id: ScopeId,
    pub parent: Option<ScopeParentInfo>,
    last_child_id: ScopeId,
    pub vars: BTreeMap<String, VarInfo>,
    pub function_store: FunctionStore,
    next_function_id: FunctionID,
} 

impl Scope {
    pub fn new_global() -> Self {
        Scope { 
            id: ScopeId(0), 
            parent: None, 
            last_child_id: ScopeId(0),
            vars: BTreeMap::new(), 
            function_store: FunctionStore::new(),
            next_function_id: FunctionID(0),
        }
    }

    pub fn new_child(parent: &Scope, allows_vars_access: bool) -> Self {
        let child_id = parent.last_child_id.increment();
        Scope { 
            id: child_id, 
            parent: Some(ScopeParentInfo { id: parent.id, allows_vars_access }), 
            last_child_id: child_id,
            vars: BTreeMap::new(), 
            function_store: FunctionStore::new(),
            next_function_id: FunctionID(0),
        }
    }

    pub fn id(&self) -> &ScopeId {
        &self.id
    }

    pub fn try_get_variable<'b>(&'b self, var_name: &String, scopes: &'b HashMap<ScopeId, Scope>) -> Option<(&'b VarInfo, ScopeId)> {
        if let Some(var) = self.vars.get(var_name) {
            return Some((var, self.id));
        }

        let mut current_scope = self;
        while let Some(parent) = &current_scope.parent {
            current_scope = scopes.get(&parent.id)?;
            
            if let Some(var) = current_scope.vars.get(var_name) {
                return Some((var, current_scope.id));
            }
        }

        None
    }

    pub fn try_get_variable_mut<'b>(self_id: &ScopeId, var_name: &String, scopes: &'b mut HashMap<ScopeId, Scope>) -> Option<&'b mut VarInfo> {
            
        let mut current_id = *self_id;

        loop {
            let (found, parent) = {
                let scope = scopes.get(&current_id)?;
                (scope.vars.contains_key(var_name), scope.parent.as_ref().map(|info| info.id.clone()))
            };

            if found {
                return scopes.get_mut(&current_id)?.vars.get_mut(var_name);
            }

            if let Some(next_parent) = parent {
                current_id = next_parent;
            } else {
                break;
            }
        }

        None
    }

    pub fn try_get_variable_current_scope_only<'b>(&'b self, var_name: &String) -> Option<&'b VarInfo> {
        if let Some(var) = self.vars.get(var_name) {
            return Some(var);
        }

        None
    }

    pub fn remove_variable_current_scope_only(&mut self, var_name: &String) -> Option<VarInfo> {
        self.vars.remove(var_name)
    }

    pub fn get_next_function_id(&mut self) -> FunctionID {
        let id = self.next_function_id.clone();
        self.next_function_id = FunctionID(self.next_function_id.0 + 1);
        id
    }

    pub fn try_get_function(
        &self,        
        name: &str,
        iter: &TokenIterator, 
        meta_data: &MetaData,
        context: &mut CurrentContext, 
        args: &Vec<ArgumentInfo>, 
        optionals: &Vec<ArgumentInfo>,
        generic_defined: &Vec<String>
    ) -> Result<Option<FunctionID>> {

        let overloaded_functions;
        match self.function_store.from_name(name) {
            Some(val) => overloaded_functions = val,
            None => return Ok(None),
        }
        
        for function in overloaded_functions {

            let mut function_call_generics = BTreeMap::<String, DefinedGenric>::new();
            for (i, (name, generic)) in function.generics.iter().enumerate() {
                if i+1 > generic_defined.len() {
                    break;
                }
                
                function_call_generics.insert(
                    name.clone(), 
                    DefinedGenric{define_type: generic_defined[i].clone(), generic: generic.clone()}
                );
            }

            context.current_generics.function_call_defined_generics = Some(function_call_generics);

            let comparable = function.are_arguments_compatible(iter, &args, &optionals, &meta_data.type_meta_data, &mut context.current_generics);
            if comparable {
                return Ok(Some(function.id));
            }
        }

        Ok(None)
    }
}














