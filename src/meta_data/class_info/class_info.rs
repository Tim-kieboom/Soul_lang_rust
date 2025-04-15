use std::collections::HashMap;
use super::{field_info::FieldInfo, methode_info::MethodeInfo};
use crate::meta_data::soul_type::generic::Generic;

#[derive(Debug, Clone)]
pub struct ClassInfo {
    pub name: String,
    pub fields: HashMap<String, FieldInfo>,
    pub methodes: HashMap<String, MethodeInfo>,
    pub generics: HashMap<String, Generic>,
}