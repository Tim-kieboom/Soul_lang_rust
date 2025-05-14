use std::{collections::HashMap, result};

use super::{c_string_store::CStringStore, class_info::class_info::ClassInfo, type_store::{TypeID, TypeStore}};

pub struct TypeMetaData {
    pub c_str_store: CStringStore,
    pub class_store: HashMap<String, ClassInfo>,
    pub type_store: TypeStore,
}

impl TypeMetaData {
    pub fn new() -> Self {
        TypeMetaData{ 
            c_str_store: CStringStore::new(),
            class_store: HashMap::new(),
            type_store: TypeStore::new(),
        }
    }

    pub fn add_type(&mut self, new_type: String) -> result::Result<TypeID, String> {
        self.type_store.add_type(new_type, None)
    }

    pub fn add_typedef(&mut self, new_type: String, from_type: String) -> result::Result<TypeID, String> {
        self.type_store.add_type(new_type, Some(from_type))
    }

    pub fn get_type_id(&self, str: &String) -> Option<TypeID> {
        self.type_store.to_id.get(str).cloned()
    }

    pub fn convert_typedef_to_original(&self, id: &TypeID) -> Option<&String> {
        self.type_store.convert_typedef_to_original(id)
    }
}










   