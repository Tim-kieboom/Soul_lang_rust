use itertools::Itertools;

use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::{soul_type::type_kind::{Modifier, TypeKind, TypeWrapper}};

#[derive(Debug, Clone, PartialEq)]
pub struct SoulType {
    pub modifier: Modifier,
    pub base: TypeKind,
    pub wrapper: Vec<TypeWrapper>,
    pub generics: Vec<SoulType>,
}

impl SoulType {
    pub fn new() -> Self {
        Self{ modifier: Modifier::Default, base: TypeKind::None, wrapper: vec![], generics: vec![] }
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




























