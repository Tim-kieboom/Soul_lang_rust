use std::collections::BTreeMap;

use crate::meta_data::{class_info::class_info::ClassInfo, scope_and_var::scope::ScopeId, soul_type::generic::Generic};
use super::{member_info::MemberInfo, rulesets::RuleSet};

#[derive(Debug, Clone)]
pub struct CurrentContext {
    rulesets: RuleSet,
    current_scope: ScopeId,
    // this_ptr: Option<VarInfo>,
    in_class: Option<ClassInfo>,
    current_generics: BTreeMap<String, Generic>,
}


