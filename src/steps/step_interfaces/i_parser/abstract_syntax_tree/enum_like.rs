use std::{collections::HashMap, thread::Scope};

use bincode::{Decode, Encode};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use crate::steps::step_interfaces::i_parser::{abstract_syntax_tree::{expression::{Expression, Ident}, generic::GenericParameter, soul_type::soul_type::SoulType, spanned::Spanned}, scope_builder::ScopeId};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Enum {
    pub name: Ident,
    pub variants: EnumVariantKind,
    pub scope_id: ScopeId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Union {
    pub name: Ident,
    pub generics: Vec<GenericParameter>,
    pub variants: Vec<Spanned<UnionVariant>>,
    pub scope_id: ScopeId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct TypeEnum {
    pub name: Ident,
    pub body: TypeEnumBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct TypeEnumBody {
    pub types: Vec<SoulType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct UnionVariant {
    pub name: Ident,
    pub field: UnionVariantKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum EnumVariantKind {
    Int(Vec<EnumVariant<i64>>),
    Expression(Vec<EnumVariant<Expression>>)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct EnumVariant<T> {
    pub name: Ident,
    pub value: T,
}








