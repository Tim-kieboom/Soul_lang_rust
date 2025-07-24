use itertools::Itertools;

use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::{soul_type::type_kind::{Modifier, TypeKind, TypeWrapper}};

#[derive(Debug, Clone, PartialEq)]
pub struct  SoulType {
    pub modifier: Modifier,
    pub base: TypeKind,
    pub wrapper: Vec<TypeWrapper>,
    pub generics: Vec<SoulType>,
}

impl SoulType {
    pub fn new() -> Self {
        Self{ modifier: Modifier::Default, base: TypeKind::None, wrapper: vec![], generics: vec![] }
    } 
    
    pub fn from_type_kind(base: TypeKind) -> Self {
        Self{ modifier: Modifier::Default, base, wrapper: vec![], generics: vec![] }
    }

    pub fn with_wrappers(mut self, wrapper: Vec<TypeWrapper>) -> Self {
        self.wrapper = wrapper;
        self    
    }

    pub fn with_mod(mut self, modifier: Modifier) -> Self {
        self.modifier = modifier;
        self    
    }

    pub fn is_none_type(&self) -> bool {
        matches!(self.base, TypeKind::None)
    }

    pub fn to_string(&self) -> String {
        if self.generics.is_empty() {
            format!(
                "{} {}{}",
                self.modifier.to_str(),
                self.base.to_string(),
                self.wrapper.iter().map(|wrap| wrap.to_str()).join("")
            )
        }
        else {
            format!(
                "{} {}<{}>{}",
                self.modifier.to_str(),
                self.base.to_string(),
                self.generics.iter().map(|gene| gene.to_string()).join(","),
                self.wrapper.iter().map(|wrap| wrap.to_str()).join("")
            )
        }
    }
}




























