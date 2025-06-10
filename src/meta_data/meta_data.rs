use bitflags::bitflags;
use std::{collections::HashMap, result, sync::{Arc, Mutex}};
use crate::{meta_data::borrow_checker::borrow_checker::{BorrowId, BorrowResult}, tokenizer::token::TokenIterator};
use crate::meta_data::soul_error::soul_error::{new_soul_error, Result};
use super::{borrow_checker::borrow_checker::{BorrowCheckedTrait, BorrowChecker, DeleteList}, class_info::class_info::ClassInfo, current_context::current_context::CurrentContext, function::{argument_info::argument_info::ArgumentInfo, function_declaration::function_declaration::{FunctionDeclaration, FunctionID}, internal_functions::INTERNAL_FUNCTIONS}, scope_and_var::{scope::{Scope, ScopeId}, var_info::VarInfo}, type_meta_data::TypeMetaData};

bitflags! {
    #[derive(Debug, PartialEq)]
    pub struct _IsFunctionResult: u8 {
        const Empty = 0;
        const IsFunction = 0b0000_0001; 
        const IsMethode = 0b0000_0010; 
    }
}

pub struct IsFunctionResult<'a> {
    pub funcs: Vec<&'a FunctionDeclaration>, 
    state: _IsFunctionResult,
}
impl<'a> IsFunctionResult<'a> {
    pub fn new(funcs: Vec<&'a FunctionDeclaration>, state: _IsFunctionResult) -> Self {
        IsFunctionResult { funcs, state }
    }
    
    pub fn is_none(&self) -> bool {
        !self.state.contains(_IsFunctionResult::IsFunction | _IsFunctionResult::IsMethode)
    }

    pub fn is_some(&self) -> bool {
        self.state.contains(_IsFunctionResult::IsFunction | _IsFunctionResult::IsMethode)
    }
    
    pub fn is_function(&self) -> bool {
        self.state.contains(_IsFunctionResult::IsFunction)
    }

    pub fn is_methode(&self) -> bool {
        self.state.contains(_IsFunctionResult::IsMethode)
    }
}

#[derive(Debug, Clone)]
pub struct FunctionStore {
    pub from_id: HashMap<FunctionID, FunctionDeclaration>,
    pub to_id: HashMap<String, Vec<FunctionID>>,
} 
impl FunctionStore {
    pub fn new() -> Self {
        FunctionStore { from_id: HashMap::new(), to_id: HashMap::new() }
    }

    pub fn len(&self) -> usize {
        self.from_id.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add_function(&mut self, name: String, id: FunctionID, func: FunctionDeclaration) {
        if let Some(ids) = self.to_id.get_mut(&name) {
            ids.push(id);
        }
        else {
            self.to_id.insert(name.to_string(), vec![id]);
        }

        self.from_id.insert(id, func);
    }

    pub fn from_name(&self, name: &str) -> Option<Vec<&FunctionDeclaration>> {
        let ids = self.to_id.get(name)?;
        Some(ids.iter().map(|id| self.from_id.get(id).unwrap()).collect::<Vec<_>>()) 
    }
}

pub struct MetaData {
    pub type_meta_data: TypeMetaData,
    pub scope_store: HashMap<ScopeId, Scope>,
    pub borrow_checker: Arc<Mutex<BorrowChecker>>,
}

const GLOBAL_SCOPE_ID: ScopeId = ScopeId(0);

impl MetaData {
    pub const GLOBAL_SCOPE_ID: ScopeId = ScopeId(0);

    pub fn new() -> Self {
        let borrow_checker = Arc::new(Mutex::new(BorrowChecker::new()));
        borrow_checker.lock().unwrap().open_scope(&MetaData::GLOBAL_SCOPE_ID).unwrap();

        let this = MetaData {  
            type_meta_data: TypeMetaData::new(), 
            scope_store: new_scope_store(),
            borrow_checker: borrow_checker,
        };

        this
    }

    pub fn add_function(&mut self, iter: &mut TokenIterator, context: &mut CurrentContext, func: FunctionDeclaration) -> Result<FunctionID> {

        let optionals: Vec<ArgumentInfo> = func.optionals.values().cloned().collect();
        if let Ok(_) = self.try_get_function(&func.name, iter, context, &func.args, &optionals, Vec::new()) {
            return Err(new_soul_error(iter.current(), format!("function: '{}' with current arguments already exists", func.name).as_str()));
        }

        let id = func.id.clone();
        let scope =self.scope_store.get_mut(&context.current_scope_id)
            .ok_or(new_soul_error(iter.current(), "Internal error: scope not found"))?;

        scope.function_store.add_function(func.name.clone(), id, func);
        Ok(id)
    }


    pub fn add_to_global_scope(&mut self, var_info: VarInfo) -> BorrowResult<()> {
        self.add_to_scope(var_info, &GLOBAL_SCOPE_ID)
    }

    pub fn add_to_scope(&mut self, var_info: VarInfo, id: &ScopeId) -> BorrowResult<()> {
        let var_name = var_info.name.clone();
        
        if !var_info.is_forward_declared {
            self.borrow_checker
                .lock().unwrap()
                .declare_owner(&BorrowId(&var_name, id))?;
        }

        self.scope_store.get_mut(id)
                        .unwrap()
                        .vars.insert(var_name, var_info);

        Ok(())
    }

    pub fn try_get_variable(&self, var_name: &String, scope_id: &ScopeId) -> Option<(&VarInfo, ScopeId)> {
        let scope = self.scope_store.get(&scope_id)?;

        scope.try_get_variable(var_name, &self.scope_store)
    }

    pub fn try_get_variable_mut(&mut self, var_name: &String, scope_id: &ScopeId) -> Option<&mut VarInfo> {
        self.scope_store.get_mut(&scope_id)?;

        Scope::try_get_variable_mut(scope_id, var_name, &mut self.scope_store)
    }

    pub fn is_variable(&self, var_name: &String, scope_id: &ScopeId) -> bool {
        self.try_get_variable(var_name, scope_id).is_some()
    }

    pub fn check_variable_valid(&self, var_name: &String, scope_id: &ScopeId) -> BorrowResult<()> {
        self.borrow_checker
            .lock().unwrap()
            .is_valid(&BorrowId(&var_name, scope_id))
    }

    pub fn try_get_function(
        &self, 
        name: &str,
        iter: &TokenIterator, 
        context: &mut CurrentContext, 
        args: &Vec<ArgumentInfo>, 
        optionals: &Vec<ArgumentInfo>,
        generic_defined: Vec<String>,
    ) -> Result<(ScopeId, FunctionID)> 
    {
        return self.internal_try_get_function(name, iter, context, args, optionals, generic_defined);
    }

    fn internal_try_get_function<'a>(
        &self, 
        name: &str,
        iter: &TokenIterator, 
        context: &mut CurrentContext, 
        args: &Vec<ArgumentInfo>, 
        optionals: &Vec<ArgumentInfo>,
        generic_defined: Vec<String>
    ) -> Result<(ScopeId, FunctionID)> {   
        let mut scope = self.scope_store.get(&context.current_scope_id).expect("Internal Error: scope_id could not be found");
        let global_scope = self.scope_store.get(&MetaData::GLOBAL_SCOPE_ID).expect("Internal Error: global scope_id could not be found");

        if name == "__soul_format_string__" {
            return Ok((MetaData::GLOBAL_SCOPE_ID, global_scope.function_store.from_name(name).unwrap()[0].id));
        }

        if let Some(function_id) = global_scope.try_get_function(name, iter, self, context, args, optionals, &generic_defined)? {
            return Ok((MetaData::GLOBAL_SCOPE_ID, function_id));
        }

        loop {
            if let Some(function_id) = scope.try_get_function(name, iter, self, context, args, optionals, &generic_defined)? {
                return Ok((*scope.id(), function_id));
            }

            if let Some(parent) = &scope.parent {
                scope = self.scope_store.get(&parent.id).expect("Internal Error: scope_id could not be found");
            }
            else {
                return Err(new_soul_error(
                    iter.current(), 
                    format!("function: '{}' not found with given arguments", name).as_str()
                ));
            }
        }
    }

    pub fn is_function<'a>(&'a self, name: &str, context: &CurrentContext) -> IsFunctionResult<'a> {
        let mut scope = self.scope_store.get(&context.current_scope_id).expect("Internal Error: scope_id could not be found");
        let global_scope = self.scope_store.get(&MetaData::GLOBAL_SCOPE_ID).expect("Internal Error: global scope_id could not be found");

        if let Some(in_class) = &context.in_class {
            
            if let Some(funcs) = global_scope.function_store.from_name(&get_methode_map_entry(name, &in_class.name)){
                return IsFunctionResult::new(funcs, _IsFunctionResult::IsFunction | _IsFunctionResult::IsMethode);
            }
        }

        if let Some(funcs) = global_scope.function_store.from_name(name) {
            return IsFunctionResult::new(funcs, _IsFunctionResult::IsFunction);
        }

        loop {
            if let Some(in_class) = &context.in_class {
                
                if let Some(funcs) = scope.function_store.from_name(&get_methode_map_entry(name, &in_class.name)){
                    return IsFunctionResult::new(funcs, _IsFunctionResult::IsFunction | _IsFunctionResult::IsMethode);
                }
            }

            if let Some(funcs) = scope.function_store.from_name(name) {
                return IsFunctionResult::new(funcs, _IsFunctionResult::IsFunction);
            }

            if let Some(parent) = &scope.parent {
                scope = self.scope_store.get(&parent.id).expect("Internal Error: scope_id could not be found");
            }
            else {
                return IsFunctionResult::new(Vec::new(), _IsFunctionResult::Empty);
            }
        }
    }

    pub fn is_methode(&self, name: &str, this_class: &ClassInfo, scope_id: &ScopeId) -> bool {
        let scope = self.scope_store.get(scope_id).expect("Internal Error: scope_id could not be found");

        scope.function_store.from_name(&get_methode_map_entry(name, &this_class.name)).is_some()
    }

    pub fn open_scope(&mut self, parent_id: ScopeId, allows_vars_access: bool, is_forward_declared: bool) -> result::Result<ScopeId, String> {
        let parent = self.scope_store.get(&parent_id)
            .ok_or("Internal Error: can not get parent scope from scope_store")?;

        let child = Scope::new_child(&parent, allows_vars_access);
        let child_id = *child.id();
        self.scope_store.insert(child_id, child);

        if is_forward_declared {
            self.borrow_checker
                .lock().unwrap()
                .open_scope(&child_id)?;
        }   
        
        Ok(child_id)
    }

    pub fn close_scope(&mut self, id: &ScopeId, is_forward_declared: bool) -> result::Result<CloseScopeResult, String> {
        if *id == MetaData::GLOBAL_SCOPE_ID {
            return Err(format!("Internal error: can not close global scope"));
        }
        
        let parent = self.scope_store.get(id)
            .ok_or(format!("Internal error: scope not found"))?
            .parent.as_ref().unwrap()
            .id.clone();
        
        self.scope_store.remove(id);
        let delete_list = if is_forward_declared {
            self.borrow_checker
                .lock().unwrap()
                .close_scope(id)?
        } 
        else {
            Vec::new()
        };

        Ok(CloseScopeResult{delete_list, parent})
    }
}

fn new_scope_store() -> HashMap<ScopeId, Scope> {
    let mut map = HashMap::new();
    let mut global_scope = Scope::new_global();

    for function in INTERNAL_FUNCTIONS.iter().cloned() {
        let id = global_scope.get_next_function_id();
        global_scope.function_store.add_function(function.name.clone(), id, function);
    }

    map.insert(*global_scope.id(), global_scope);
    map
}

fn get_methode_map_entry(func_name: &str, this_class: &str) -> String {
    format!("{}#{}", this_class, func_name)
}

pub struct CloseScopeResult {
    pub parent: ScopeId, 
    pub delete_list: DeleteList
}



