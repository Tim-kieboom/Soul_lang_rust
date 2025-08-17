use serde::{Deserialize, Serialize};

use crate::{steps::step_interfaces::{i_parser::abstract_syntax_tree::{expression::{Expression, Ident}, generics::GenericParam, soul_type::soul_type::SoulType, spanned::Spanned, staments::function::{FnDeclKind, FunctionSignatureRef}, visibility::FieldAccess}, i_sementic::sementic_scope::Byte}, utils::node_ref::{FromPoolValue, MultiRef}};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraitImpl {
    pub trait_name: Ident,
    pub for_type: SoulType,
    pub methodes: Vec<Spanned<FnDeclKind>>,
}
pub type TraitDeclRef = MultiRef<InnerTraitDecl>;
impl FromPoolValue for InnerTraitDecl {
    fn is_from_pool_value(from: &crate::utils::node_ref::PoolValue) -> bool {
        match from {
            crate::utils::node_ref::PoolValue::Trait(inner_trait_decl) => true,
            _ => false,
        }
    }

    fn from_pool_value_mut(from: &mut crate::utils::node_ref::PoolValue) -> &mut Self {
        match from {
            crate::utils::node_ref::PoolValue::Trait(inner_trait_decl) => inner_trait_decl,
            _ => panic!("PoolValue is wrong type"),
        }
    }

    fn from_pool_value_ref(from: &crate::utils::node_ref::PoolValue) -> &Self {
        match from {
            crate::utils::node_ref::PoolValue::Trait(inner_trait_decl) => inner_trait_decl,
            _ => panic!("PoolValue is wrong type"),
        }
    }

    fn to_pool_value(self) -> crate::utils::node_ref::PoolValue {
        crate::utils::node_ref::PoolValue::Trait(self)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerTraitDecl {
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub methodes: Vec<FunctionSignatureRef>,
}

pub type StructDeclRef = MultiRef<InnerStructDecl>;
impl FromPoolValue for InnerStructDecl {
    fn is_from_pool_value(from: &crate::utils::node_ref::PoolValue) -> bool {
        match from {
            crate::utils::node_ref::PoolValue::Struct(inner_struct_decl) => true,
            _ => false,
        }
    }

    fn from_pool_value_mut(from: &mut crate::utils::node_ref::PoolValue) -> &mut Self {
        match from {
            crate::utils::node_ref::PoolValue::Struct(inner_struct_decl) => inner_struct_decl,
            _ => panic!("PoolValue is wrong type")
        }
    }

    fn from_pool_value_ref(from: &crate::utils::node_ref::PoolValue) -> &Self {
        match from {
            crate::utils::node_ref::PoolValue::Struct(inner_struct_decl) => inner_struct_decl,
            _ => panic!("PoolValue is wrong type")
        }
    }

    fn to_pool_value(self) -> crate::utils::node_ref::PoolValue {
        crate::utils::node_ref::PoolValue::Struct(self)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerStructDecl {
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub fields: Vec<Spanned<FieldDecl>>,
    pub size: Byte,
}

pub type ClassDeclRef = MultiRef<InnerClassDecl>;
impl FromPoolValue for InnerClassDecl {
    fn is_from_pool_value(from: &crate::utils::node_ref::PoolValue) -> bool {
        match from {
            crate::utils::node_ref::PoolValue::Class(inner_class_decl) => true,
            _ => false,
        }
    }

    fn from_pool_value_mut(from: &mut crate::utils::node_ref::PoolValue) -> &mut Self {
        match from {
            crate::utils::node_ref::PoolValue::Class(inner_class_decl) => inner_class_decl,
            _ => panic!("PoolValue is wrong type"),
        }
    }

    fn from_pool_value_ref(from: &crate::utils::node_ref::PoolValue) -> &Self {
        match from {
            crate::utils::node_ref::PoolValue::Class(inner_class_decl) => inner_class_decl,
            _ => panic!("PoolValue is wrong type"),
        }
    }

    fn to_pool_value(self) -> crate::utils::node_ref::PoolValue {
        crate::utils::node_ref::PoolValue::Class(self)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerClassDecl {
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub fields: Vec<Spanned<FieldDecl>>,
    pub methodes: Vec<Spanned<FnDeclKind>>,
    pub size: Byte,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldDecl {
    pub name: Ident,
    pub ty: SoulType,
    pub default_value: Option<Expression>,
    pub vis: FieldAccess
}

impl FieldDecl {
    pub fn new_struct_field(name: Ident, ty: SoulType) -> Self {
        Self{name, ty, default_value: None, vis: FieldAccess::default() }
    }
}






















