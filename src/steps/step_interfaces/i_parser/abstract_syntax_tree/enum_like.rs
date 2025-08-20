use itertools::Itertools;
use serde::{Deserialize, Serialize};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::{Ident, NamedTuple, Tuple}, soul_type::soul_type::SoulType};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enum {
    pub name: Ident,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Union {
    pub name: Ident,
    pub variants: Vec<UnionVariant>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeEnum {
    pub name: Ident,
    pub types: Vec<SoulType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnionVariant {
    pub name: Ident,
    pub field: UnionVariantKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnionVariantKind {
    Tuple(Tuple),
    NamedTuple(NamedTuple)
}

impl UnionVariantKind {
    pub fn to_string(&self) -> String {
        match self {
            UnionVariantKind::Tuple(soul_types) => format!("({})", soul_types.values.iter().map(|el| el.to_string()).join(",")),
            UnionVariantKind::NamedTuple(hash_map) => format!("({})", hash_map.values.iter().map(|(name, el)| format!("{}: {}", name.0, el.node.to_string())).join(",")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: Ident,
    pub value: i64,
}








