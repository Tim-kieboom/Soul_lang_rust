use bitflags::bitflags;
use itertools::Itertools;
use std::{cmp::Ordering, collections::{BTreeMap, BTreeSet, HashMap, HashSet}, io::Result, ops::{Index, IndexMut}, result, sync::{Arc, Mutex}};
use crate::tokenizer::token::TokenIterator;

use super::{borrow_checker::borrow_checker::{BorrowCheckedTrait, BorrowChecker, DeleteList}, class_info::class_info::ClassInfo, convert_soul_error::convert_soul_error::new_soul_error, current_context::current_context::{CurrentContext, DefinedGenric}, function::{argument_info::argument_info::ArgumentInfo, function_declaration::function_declaration::{FunctionDeclaration, FunctionID}, internal_functions::{FIRST_FUNCTION_ID, INTERNAL_FUNCTIONS}}, scope_and_var::{scope::{Scope, ScopeId}, var_info::VarInfo}, soul_type::generic::Generic, type_meta_data::TypeMetaData};

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

#[derive(Debug)]
pub struct FunctionStore {
    pub from_id: HashMap<FunctionID, FunctionDeclaration>,
    pub to_id: HashMap<String, Vec<FunctionID>>,
} 
impl FunctionStore {
    pub fn new() -> Self {
        FunctionStore { from_id: HashMap::new(), to_id: HashMap::new() }
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
    pub function_store: FunctionStore,
    pub borrow_checker: Arc<Mutex<BorrowChecker>>,
    next_function_id: FunctionID,
}

const GLOBAL_SCOPE_ID: ScopeId = ScopeId(0);

impl MetaData {
    pub const GLOBAL_SCOPE_ID: ScopeId = ScopeId(0);

    pub fn new() -> Self {
        let borrow_checker = Arc::new(Mutex::new(BorrowChecker::new()));
        borrow_checker.lock().unwrap().open_scope(&MetaData::GLOBAL_SCOPE_ID);

        let mut this = MetaData {  
            type_meta_data: TypeMetaData::new(), 
            scope_store: new_scope_store(&borrow_checker),
            function_store: FunctionStore::new(),
            borrow_checker: borrow_checker,
            next_function_id: FunctionID(0),
        };

        for function in INTERNAL_FUNCTIONS.iter().cloned() {
            let id = this.get_next_function_id();
            this.function_store.add_function(function.name.clone(), id, function);
        }

        this
    }

    pub fn add_function(&mut self, iter: &mut TokenIterator, context: &mut CurrentContext, func: FunctionDeclaration) -> Result<FunctionID> {

        let optionals: Vec<ArgumentInfo> = func.optionals.values().cloned().collect();
        if let Ok(_) = self.try_get_function(&func.name, iter, context, &func.args, &optionals, Vec::new()) {
            return Err(new_soul_error(iter.current(), format!("function: '{}' with current arguments already exists", func.name).as_str()));
        }

        let id = func.id.clone();
        self.function_store.add_function(func.name.clone(), id, func);
        Ok(id)
    }

    pub fn get_next_function_id(&mut self) -> FunctionID {
        let id = self.next_function_id.clone();
        self.next_function_id = FunctionID(self.next_function_id.0 + 1);
        id
    }

    pub fn add_to_global_scope(&mut self, var_info: VarInfo) {
        self.add_to_scope(var_info, &GLOBAL_SCOPE_ID);
    }

    pub fn add_to_scope(&mut self, var_info: VarInfo, id: &ScopeId) {
        self.scope_store.get_mut(id)
                        .unwrap()
                        .vars.insert(var_info.name.clone(), var_info);
    }

    pub fn try_get_variable(&self, var_name: &String, scope_id: &ScopeId) -> Option<&VarInfo> {
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

    pub fn try_get_function(
        &self, 
        name: &str,
        iter: &TokenIterator, 
        context: &mut CurrentContext, 
        args: &Vec<ArgumentInfo>, 
        optionals: &Vec<ArgumentInfo>,
        generic_defined: Vec<String>,
    ) -> Result<FunctionID> 
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
    ) -> Result<FunctionID> {        
        let overloaded_functions = self.function_store.from_name(name)
            .ok_or(new_soul_error(iter.current(), format!("function: '{}' is not found", name).as_str()))?;

        if name == "__Soul_format_string__" {
            return Ok(overloaded_functions[0].id);
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

            let comparable = function.are_arguments_compatible(iter, &args, &optionals, &self.type_meta_data, &mut context.current_generics);
            if comparable {
                return Ok(function.id);
            }
        }

        return Err(new_soul_error(
            iter.current(), 
            format!("function: '{}' not found with given arguments", name).as_str()
        ));
    }

    pub fn is_function<'a>(&'a self, name: &str, context: &CurrentContext) -> IsFunctionResult<'a> {
        if let Some(in_class) = &context.in_class {
            
            if let Some(funcs) = self.function_store.from_name(&get_methode_map_entry(name, &in_class.name)){
                return IsFunctionResult::new(funcs, _IsFunctionResult::IsFunction | _IsFunctionResult::IsMethode);
            }
        }

        if let Some(funcs) = self.function_store.from_name(name) {
            return IsFunctionResult::new(funcs, _IsFunctionResult::IsFunction);
        }
        else {
            return IsFunctionResult::new(Vec::new(), _IsFunctionResult::Empty);
        }
    }

    pub fn is_methode(&self, name: &str, this_class: &ClassInfo) -> bool {
        self.function_store.from_name(&get_methode_map_entry(name, &this_class.name)).is_some()
    }

    pub fn open_scope(&mut self, parent_id: ScopeId) -> result::Result<ScopeId, String> {
        let parent = self.scope_store.get(&parent_id)
            .ok_or("Internal Error: can not get parent scope from scope_store")?;
        let child = Scope::new_child(Arc::clone(&self.borrow_checker), &parent);
        let child_id = *child.id();
        self.scope_store.insert(child_id, child);
        self.borrow_checker.lock().unwrap().open_scope(&child_id)?;

        Ok(child_id)
    }

    pub fn close_scope(&mut self, id: &ScopeId) -> result::Result<DeleteList, String> {
        self.scope_store.remove(id);
        self.borrow_checker.lock().unwrap().close_scope(id)
    }

}

fn new_scope_store(borrow_checker: &Arc<Mutex<BorrowChecker>>) -> HashMap<ScopeId, Scope> {
    let mut map = HashMap::new();
    let global_scope = Scope::new_global(Arc::clone(borrow_checker));
    map.insert(*global_scope.id(), global_scope);
    map
}

fn get_methode_map_entry(func_name: &str, this_class: &str) -> String {
    format!("{}#{}", this_class, func_name)
}
