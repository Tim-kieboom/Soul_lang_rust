use itertools::Itertools;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::{steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::Ident, soul_type::soul_type::SoulType}, utils::node_ref::MultiRef};

pub type EnumDeclRef = MultiRef<InnerEnumDecl>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerEnumDecl {
    pub name: Ident,
    pub variants: Vec<EnumVariant>,
    
    ///for determenint if internal type should be in or uint
    pub min_num: i64,
    ///for determenint how big inner type should be
    pub max_num: i64,
}

pub type UnionDeclRef = MultiRef<InnerUnionDecl>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerUnionDecl {
    pub name: Ident,
    pub variants: Vec<UnionVariant>,
    pub byte_size: usize,
}

pub type TypeEnumDeclRef = MultiRef<InnerTypeEnumDecl>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerTypeEnumDecl {
    pub name: Ident,
    pub types: Vec<SoulType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: Ident,
    pub value: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnionVariant {
    pub name: Ident,
    pub field: UnionVariantKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnionVariantKind {
    Tuple(Vec<SoulType>),
    NamedTuple(HashMap<Ident, SoulType>)
}

impl UnionVariantKind {
    pub fn to_string(&self) -> String {
        match self {
            UnionVariantKind::Tuple(soul_types) => format!("({})", soul_types.iter().map(|el| el.to_string()).join(",")),
            UnionVariantKind::NamedTuple(hash_map) => format!("({})", hash_map.iter().map(|(name, el)| format!("{}: {}", name.0, el.to_string())).join(",")),
        }
    }
}



















