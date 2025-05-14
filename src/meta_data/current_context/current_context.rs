use std::collections::BTreeMap;

use crate::meta_data::{class_info::class_info::ClassInfo, scope_and_var::scope::ScopeId, soul_type::generic::Generic};
use super::{member_info::MemberInfo, rulesets::RuleSet};

#[derive(Debug, Clone)]
pub struct CurrentContext {
    pub rulesets: RuleSet,
    pub current_scope_id: ScopeId,
    // this_ptr: Option<VarInfo>,
    pub in_class: Option<ClassInfo>,
    pub current_generics: BTreeMap<String, Generic>,
}

impl CurrentContext {
    pub fn new(current_scope_id: ScopeId) -> Self {
        CurrentContext { 
            rulesets: RuleSet::Default, 
            current_scope_id, 
            in_class: None, 
            current_generics: BTreeMap::new(),
        }
    }
}

