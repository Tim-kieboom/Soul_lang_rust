use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::{Ident, NamedTuple, Tuple}, soul_type::soul_type::SoulType};

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDecl {
    pub name: Ident,
    pub variants: Vec<EnumVariant>,
    
    ///for determenint if internal type should be in or uint
    pub min_num: i64,
    ///for determenint how big inner type should be
    pub max_num: i64,
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

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub name: Ident,
    pub value: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionVariant {
    pub name: Ident,
    pub fields: Vec<UnionVariantKind>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnionVariantKind {
    Tuple(Tuple),
    NamedTuple(NamedTuple)
}





















