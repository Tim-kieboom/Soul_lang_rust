use itertools::Itertools;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::{steps::step_interfaces::{i_parser::abstract_syntax_tree::{expression::Ident, soul_type::soul_type::SoulType}, i_sementic::sementic_scope::Byte}, utils::serde_multi_ref::{MultiRef, MultiRefPool}};

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
    pub size: Byte,
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
    pub fn iter_types(&self) -> Box<dyn Iterator<Item = &SoulType> + '_> {
        match self {
            UnionVariantKind::Tuple(soul_types) => Box::new(soul_types.iter()),
            UnionVariantKind::NamedTuple(hash_map) => Box::new(hash_map.values()),
        }
    }
    
    pub fn to_string(&self, ref_pool: &MultiRefPool) -> String {
        match self {
            UnionVariantKind::Tuple(soul_types) => format!("({})", soul_types.iter().map(|el| el.to_string(ref_pool)).join(",")),
            UnionVariantKind::NamedTuple(hash_map) => format!("({})", hash_map.iter().map(|(name, el)| format!("{}: {}", name.0, el.to_string(ref_pool))).join(",")),
        }
    }
}



















