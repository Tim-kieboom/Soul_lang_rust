use super::rulesets::RuleSet;
use std::collections::BTreeMap;
use crate::meta_data::{class_info::class_info::ClassInfo, function::function_declaration::function_declaration::FunctionDeclaration, scope_and_var::scope::ScopeId, soul_type::generic::Generic};

#[derive(Debug, Clone, PartialEq)]
pub struct DefinedGenric {
    pub define_type: String,
    pub generic: Generic,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CurrentGenerics {
    pub scope_generics: BTreeMap<String, Generic>,
    pub function_call_defined_generics: Option<BTreeMap<String, DefinedGenric>>,
}

impl CurrentGenerics {
    pub fn new() -> Self {
        CurrentGenerics { scope_generics: BTreeMap::new(), function_call_defined_generics: None }
    }

    pub fn is_function_call_defined_generic(&mut self, name: &String) -> bool { 
        self.function_call_defined_generics.as_ref().is_some_and(|generics| generics.contains_key(name))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CurrentContext {
    pub rulesets: RuleSet,
    // this_ptr: Option<VarInfo>,
    pub current_scope_id: ScopeId,
    pub in_class: Option<ClassInfo>,
    pub current_generics: CurrentGenerics,
    pub current_function: Option<FunctionDeclaration>,
}

impl CurrentContext {
    pub fn new(current_scope_id: ScopeId) -> Self {
        CurrentContext { 
            in_class: None, 
            current_scope_id, 
            current_function: None,
            rulesets: RuleSet::Default, 
            current_generics: CurrentGenerics::new(),
        }
    }
}






