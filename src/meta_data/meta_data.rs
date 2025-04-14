use std::collections::HashMap;

use super::{key_tokens::{SoulNameEnum, SoulNames}, scope_and_var::{scope::{Scope, ScopeId}, var_info::{self, VarInfo}}, type_meta_data::TypeMetaData};

pub struct MetaData<'a> {
    pub soul_names: SoulNames<'a>,
    pub type_meta_data: TypeMetaData,
    pub scope_store: HashMap<ScopeId, Scope>,
}

const GLOBAL_SCOPE_ID: ScopeId = ScopeId(0);

impl<'a> MetaData<'a> {
    
    pub fn new() -> Self {
        MetaData { 
            soul_names: SoulNames::new(), 
            type_meta_data: TypeMetaData::new(), 
            scope_store: new_scope_store(),
        }
    }

    pub fn get_soul_name<T: SoulNameEnum<'a>>(&self, key: T) -> &'a str {
        key.get(&self.soul_names).expect("some of the template soul_names are not implemented")
    }

    pub fn add_to_global_scope(&mut self, var_info: VarInfo) {
        self.scope_store.get_mut(&GLOBAL_SCOPE_ID)
                        .unwrap()
                        .vars.insert(var_info.name.clone(), var_info);
    }
}

fn new_scope_store() -> HashMap<ScopeId, Scope> {
    let mut map = HashMap::new();
    let global_scope = Scope::new_global();
    map.insert(*global_scope.id(), global_scope);
    map
}