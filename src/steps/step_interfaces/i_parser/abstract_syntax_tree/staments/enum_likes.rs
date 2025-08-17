use itertools::Itertools;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::{steps::step_interfaces::{i_parser::abstract_syntax_tree::{expression::Ident, soul_type::soul_type::SoulType}, i_sementic::sementic_scope::Byte}, utils::node_ref::{FromPoolValue, MultiRef, MultiRefPool}};

pub type EnumDeclRef = MultiRef<InnerEnumDecl>;
impl FromPoolValue for InnerEnumDecl {
    fn is_from_pool_value(from: &crate::utils::node_ref::PoolValue) -> bool {
        match from {
            crate::utils::node_ref::PoolValue::Enum(inner_enum_decl) => true,
            _ => false,
        }
    }

    fn from_pool_value_mut(from: &mut crate::utils::node_ref::PoolValue) -> &mut Self {
        match from {
            crate::utils::node_ref::PoolValue::Enum(inner_enum_decl) => inner_enum_decl,
            _ => panic!("PoolValue is wrong type"),
        }
    }

    fn from_pool_value_ref(from: &crate::utils::node_ref::PoolValue) -> &Self {
        match from {
            crate::utils::node_ref::PoolValue::Enum(inner_enum_decl) => inner_enum_decl,
            _ => panic!("PoolValue is wrong type"),
        }
    }

    fn to_pool_value(self) -> crate::utils::node_ref::PoolValue {
        crate::utils::node_ref::PoolValue::Enum(self)
    }
}

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
impl FromPoolValue for InnerUnionDecl {
    fn is_from_pool_value(from: &crate::utils::node_ref::PoolValue) -> bool {
        match from {
            crate::utils::node_ref::PoolValue::Union(inner_union_decl) => true,
            _ => false,
        }
    }

    fn from_pool_value_mut(from: &mut crate::utils::node_ref::PoolValue) -> &mut Self {
        match from {
            crate::utils::node_ref::PoolValue::Union(inner_union_decl) => inner_union_decl,
            _ => panic!("PoolValue is wrong type"),
        }
    }

    fn from_pool_value_ref(from: &crate::utils::node_ref::PoolValue) -> &Self {
        match from {
            crate::utils::node_ref::PoolValue::Union(inner_union_decl) => inner_union_decl,
            _ => panic!("PoolValue is wrong type"),
        }
    }

    fn to_pool_value(self) -> crate::utils::node_ref::PoolValue {
        crate::utils::node_ref::PoolValue::Union(self)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerUnionDecl {
    pub name: Ident,
    pub variants: Vec<UnionVariant>,
    pub size: Byte,
}

pub type TypeEnumDeclRef = MultiRef<InnerTypeEnumDecl>;
impl FromPoolValue for InnerTypeEnumDecl {
    fn is_from_pool_value(from: &crate::utils::node_ref::PoolValue) -> bool {
        match from {
            crate::utils::node_ref::PoolValue::TypeEnum(_) => true,
            _ => false,
        }
    }

    fn from_pool_value_mut(from: &mut crate::utils::node_ref::PoolValue) -> &mut Self {
        match from {
            crate::utils::node_ref::PoolValue::TypeEnum(inner_type_enum_decl) => inner_type_enum_decl,
            _ => panic!("PoolValue is wrong type"),
        }
    }

    fn from_pool_value_ref(from: &crate::utils::node_ref::PoolValue) -> &Self {
        match from {
            crate::utils::node_ref::PoolValue::TypeEnum(inner_type_enum_decl) => inner_type_enum_decl,
            _ => panic!("PoolValue is wrong type"),
        }
    }

    fn to_pool_value(self) -> crate::utils::node_ref::PoolValue {
        crate::utils::node_ref::PoolValue::TypeEnum(self)
    }
}

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



















