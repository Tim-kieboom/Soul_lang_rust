use itertools::Itertools;
use serde::{Deserialize, Serialize};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::Ident, soul_type::soul_type::SoulType};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericParameter {
    pub name: Ident,
    pub constraint: Vec<TypeConstraint>,
    pub kind: GenericKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GenericKind {
    Type{default: Option<SoulType>},
    Lifetime
}

impl GenericParameter {
    pub fn to_string(&self) -> String {
        let str = match self.constraint.is_empty() {
            true => format!("{}", self.name.0),
            false => format!("{}: {}", self.name.0, self.constraint.iter().map(|ty| ty.to_string()).join("+")),
        };

        match &self.kind {
            GenericKind::Type { default } => match default {
                Some(val) => format!("{} = {}", str, val.to_string()),
                None => str,
            },
            GenericKind::Lifetime => str,
        }
    } 
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeConstraint {
    Type(SoulType),
    LiteralTypeEnum(Vec<SoulType>),
}

impl TypeConstraint {
    pub fn to_string(&self) -> String {
        match self {
            TypeConstraint::Type(ty) => ty.to_string(),
            TypeConstraint::LiteralTypeEnum(soul_types) => format!("typeof[{}]", soul_types.iter().map(|ty| ty.to_string()).join(",")),
        }
    }
}








