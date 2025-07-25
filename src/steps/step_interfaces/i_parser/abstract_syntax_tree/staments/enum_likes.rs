use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::Ident, soul_type::{soul_type::SoulType, type_kind::{EnumVariant, UnionVariant}}};

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDecl {
    pub name: Ident,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionDecl {
    pub name: Ident,
    pub variants: Vec<UnionVariant>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeEnumDecl {
    pub name: Ident,
    pub types: Vec<SoulType>,
}























