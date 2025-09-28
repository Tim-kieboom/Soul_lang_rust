use bincode::{Decode, Encode};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::{enum_like::TypeEnumBody, expression::{Ident}, soul_type::soul_type::SoulType};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct GenericParameter {
    pub name: Ident,
    pub constraint: Vec<TypeConstraint>,
    pub kind: GenericKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum GenericKind {
    Type{impl_type: Option<SoulType>, default: Option<SoulType>},
    Lifetime
}

impl GenericParameter {
    pub fn to_string(&self) -> String {
        let str = match self.constraint.is_empty() {
            true => format!("{}", self.name.0),
            false => format!("{}: {}", self.name.0, self.constraint.iter().map(|ty| ty.to_string()).join("+")),
        };

        match &self.kind {
            GenericKind::Type{impl_type, default} => match default {
                Some(val) => format!(
                    "{}{} = {}", 
                    str,
                    impl_type.as_ref().map(|el| format!("impl {}", el.to_string())).unwrap_or("".into()),
                    val.to_string(),
                ),
                None => str,
            },
            GenericKind::Lifetime => str,
        }
    } 
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum TypeConstraint {
    Type(SoulType),
    LiteralTypeEnum(TypeEnumBody),
}

impl TypeConstraint {
    pub fn to_string(&self) -> String {
        match self {
            TypeConstraint::Type(ty) => ty.to_string(),
            TypeConstraint::LiteralTypeEnum(soul_types) => format!(
                "typeof[{}]", 
                soul_types.types.iter().map(|ty| ty.to_string()).join(","), 
            ),
        }
    }
}








