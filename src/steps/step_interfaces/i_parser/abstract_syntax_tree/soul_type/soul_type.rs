use itertools::Itertools;
use serde::{Deserialize, Serialize};
use crate::{steps::step_interfaces::i_parser::abstract_syntax_tree::{soul_type::type_kind::{Modifier, TypeKind, TypeWrapper}, staments::statment::Lifetime}, utils::node_ref::MultiRefPool};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeGenericKind {
    Type(SoulType),
    Lifetime(Lifetime)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct  SoulType {
    pub modifier: Modifier,
    pub base: TypeKind,
    pub wrappers: Vec<TypeWrapper>,
    pub generics: Vec<TypeGenericKind>,
}

impl TypeGenericKind {
    pub fn to_string(&self, ref_pool: &MultiRefPool) -> String {
        match self {
            TypeGenericKind::Type(soul_type) => soul_type.to_string(ref_pool),
            TypeGenericKind::Lifetime(lifetime) => lifetime.name.0.clone(),
        }
    }
}

impl SoulType {
    pub fn none() -> Self {
        Self{ modifier: Modifier::Default, base: TypeKind::None, wrappers: vec![], generics: vec![] }
    } 
    
    pub fn from_type_kind(base: TypeKind) -> Self {
        Self{ modifier: Modifier::Default, base, wrappers: vec![], generics: vec![] }
    }

    pub fn with_wrappers(mut self, wrapper: Vec<TypeWrapper>) -> Self {
        self.wrappers = wrapper;
        self    
    }

    pub fn with_mod(mut self, modifier: Modifier) -> Self {
        self.modifier = modifier;
        self    
    }

    pub fn is_none_type(&self) -> bool {
        matches!(self.base, TypeKind::None)
    }

    pub fn to_deref(&self, ref_pool: &MultiRefPool) -> Result<Self, String> {
        if self.wrappers.is_empty() {
            return Err(format!("type: '{}' can not be derefed", self.to_string(ref_pool)))
        }

        let mut this = self.clone();
        match this.wrappers.pop().unwrap() {
            TypeWrapper::Invalid |
            TypeWrapper::Array => Err(format!("type: '{}' can not be derefed", self.to_string(ref_pool))),
            TypeWrapper::Pointer |
            TypeWrapper::MutRef(_) |
            TypeWrapper::ConstRef(_) |
            TypeWrapper::ConstPointer => Ok(this),
        }
    }

    pub fn to_string(&self, ref_pool: &MultiRefPool) -> String {
        if self.generics.is_empty() {
            format!(
                "{} {}{}",
                self.modifier.to_str(),
                self.base.to_string(ref_pool),
                self.wrappers.iter().map(|wrap| wrap.to_str()).join("")
            )
        }
        else {
            format!(
                "{} {}<{}>{}",
                self.modifier.to_str(),
                self.base.to_string(ref_pool),
                self.generics.iter().map(|gene| gene.to_string(ref_pool)).join(","),
                self.wrappers.iter().map(|wrap| wrap.to_str()).join("")
            )
        }
    }
}




























