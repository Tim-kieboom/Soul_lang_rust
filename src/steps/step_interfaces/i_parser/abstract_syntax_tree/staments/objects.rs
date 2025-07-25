use crate::{steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::{Expression, Ident}, generics::GenericParam, soul_type::soul_type::SoulType, spanned::Spanned, staments::function::{FnDecl, FnDeclKind, FunctionSignatureRef}, visibility::FieldAccess}, utils::node_ref::NodeRef};

#[derive(Debug, Clone, PartialEq)]
pub struct TraitImpl {
    pub trait_name: Ident,
    pub for_type: SoulType,
    pub methodes: Vec<FnDecl>,
}
pub type TraitDeclRef = NodeRef<InnerTraitDecl>;

#[derive(Debug, Clone, PartialEq)]
pub struct InnerTraitDecl {
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub methodes: Vec<FunctionSignatureRef>,
    pub implements: Vec<Ident>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDecl {
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub fields: Vec<FieldDecl>,
    pub implements: Vec<Ident>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDecl {
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub fields: Vec<FieldDecl>,
    pub methodes: Vec<Spanned<FnDeclKind>>,
    pub implements: Vec<Ident>,
}

#[derive(Debug, Clone, PartialEq)]
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






















