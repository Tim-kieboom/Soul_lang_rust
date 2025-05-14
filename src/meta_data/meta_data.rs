use bitflags::bitflags;
use std::{collections::HashMap, io::Result};
use crate::tokenizer::token::TokenIterator;

use super::{class_info::class_info::ClassInfo, convert_soul_error::convert_soul_error::new_soul_error, current_context::current_context::CurrentContext, function::{argument_info::argument_info::ArgumentInfo, function_declaration::function_declaration::{FunctionDeclaration, FunctionID}, internal_functions::{FIRST_FUNCTION_ID, INTERNAL_FUNCTIONS}}, scope_and_var::{scope::{Scope, ScopeId}, var_info::VarInfo}, type_meta_data::TypeMetaData};

bitflags! {
    #[derive(Debug, PartialEq)]
    pub struct IsFunctionResult: u8 {
        const None = 0b0000_0000;
        const IsFunction = 0b0000_0001; 
        const IsMethode = 0b0000_0010; 
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
    next_function_id: FunctionID,
}

const GLOBAL_SCOPE_ID: ScopeId = ScopeId(0);

impl MetaData {
    pub const GLOBAL_SCOPE_ID: ScopeId = ScopeId(0);

    pub fn new() -> Self {
        let mut this = MetaData {  
            type_meta_data: TypeMetaData::new(), 
            scope_store: new_scope_store(),
            function_store: FunctionStore::new(),
            next_function_id: FunctionID(0),
        };

        for function in INTERNAL_FUNCTIONS.iter().cloned() {
            let id = this.get_next_function_id();
            this.function_store.add_function(function.name.clone(), id, function);
        }

        this
    }

    pub fn add_function(&mut self, iter: &mut TokenIterator, context: &CurrentContext, func: FunctionDeclaration) -> Result<FunctionID> {

        let optionals: Vec<ArgumentInfo> = func.optionals.values().cloned().collect();
        if let Ok(_) = self.try_get_function(&func.name, iter, context, &func.args, &optionals) {
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
        self.scope_store.get_mut(&GLOBAL_SCOPE_ID)
                        .unwrap()
                        .vars.insert(var_info.name.clone(), var_info);
    }

    pub fn add_to_scope(&mut self, var_info: VarInfo, id: &ScopeId) {
        self.scope_store.get_mut(id)
                        .unwrap()
                        .vars.insert(var_info.name.clone(), var_info);
    }

    pub fn try_get_variable(&self, var_name: &String, scope_id: &ScopeId) -> Option<&VarInfo> {
        let scope = &self.scope_store.get(&scope_id)?;

        scope.try_get_variable(var_name, &self.scope_store)
    }

    pub fn is_variable(&self, var_name: &String, scope_id: &ScopeId) -> bool {
        self.try_get_variable(var_name, scope_id).is_some()
    }

    pub fn try_get_function(
        &self, 
        name: &str,
        iter: &mut TokenIterator, 
        context: &CurrentContext, 
        args: &Vec<ArgumentInfo>, 
        optionals: &Vec<ArgumentInfo>,
    ) -> Result<FunctionID> 
    {
        return self.internal_try_get_function(name, iter, context, args, optionals);
    }

    fn internal_try_get_function(
        &self, 
        name: &str,
        iter: &mut TokenIterator, 
        context: &CurrentContext, 
        args: &Vec<ArgumentInfo>, 
        optionals: &Vec<ArgumentInfo>,
    ) -> Result<FunctionID> {
        let overloaded_functions = self.function_store.from_name(name)
            .ok_or(new_soul_error(iter.current(), format!("function: '{}' is not found", name).as_str()))?;
        
        for function in overloaded_functions {
            let are_compatible = function.are_arguments_compatible(iter, &args, &optionals, &self.type_meta_data, &context.current_generics);
            if are_compatible {
                return Ok(function.id);
            }
        }

        Err(new_soul_error(
            iter.current(), 
            format!("function: '{}' not found with given arguments", name).as_str()
        ))
    }

    pub fn is_function(&self, name: &str, context: &CurrentContext) -> IsFunctionResult {
        if context.in_class.is_some() && self.is_methode(name, &context.in_class.as_ref().unwrap()) {
            return IsFunctionResult::IsFunction | IsFunctionResult::IsMethode;
        }

        if self.function_store.from_name(name).is_some() {
            return IsFunctionResult::IsFunction;
        }
        else {
            return IsFunctionResult::None;
        }
    }

    pub fn is_methode(&self, name: &str, this_class: &ClassInfo) -> bool {
        self.function_store.from_name(&get_methode_map_entry(name, &this_class.name)).is_some()
    }

    pub fn new_scope(&mut self, parent_id: ScopeId) -> Option<ScopeId> {
        let parent = self.scope_store.get(&parent_id)?;
        let child = Scope::new_child(&parent);
        let child_id = *child.id();
        self.scope_store.insert(child_id, child);

        Some(child_id)
    }
}

fn new_scope_store() -> HashMap<ScopeId, Scope> {
    let mut map = HashMap::new();
    let global_scope = Scope::new_global();
    map.insert(*global_scope.id(), global_scope);
    map
}

fn get_methode_map_entry(func_name: &str, this_class: &str) -> String {
    format!("{}#{}", this_class, func_name)
}






