use serde::{Deserialize, Serialize};

use crate::{steps::step_interfaces::{i_parser::abstract_syntax_tree::{expression::{Expression, Ident}, generics::GenericParam, soul_type::soul_type::SoulType, spanned::Spanned, staments::function::{FnDeclKind, FunctionSignatureRef}, visibility::FieldAccess}, i_sementic::sementic_scope::Byte}, utils::serde_multi_ref::{MultiRef}};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraitImpl {
    pub trait_name: Ident,
    pub for_type: SoulType,
    pub methodes: Vec<Spanned<FnDeclKind>>,
}
pub type TraitDeclRef = MultiRef<InnerTraitDecl>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerTraitDecl {
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub methodes: Vec<FunctionSignatureRef>,
}

pub type StructDeclRef = MultiRef<InnerStructDecl>;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerStructDecl {
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub fields: Vec<Spanned<FieldDecl>>,
    pub size: Byte,
}

pub type ClassDeclRef = MultiRef<InnerClassDecl>;

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






















