use itertools::Itertools;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::Ident, soul_type::soul_type::SoulType};

#[derive(Debug, Clone, PartialEq)]
pub struct GenericParam {
    pub name: Ident,
    pub constraint: Vec<TypeConstraint>,
    pub kind: GenericKind,
    pub default: Option<SoulType>, //only for typeGenerics
}

#[derive(Debug, Clone, PartialEq)]
pub enum GenericKind {
    Type,
    Lifetime
}

impl GenericParam {
    pub fn to_string(&self) -> String {
        let str = match self.constraint.is_empty() {
            true => format!("{}", self.name.0),
            false => format!("{}: {}", self.name.0, self.constraint.iter().map(|ty| ty.to_string()).join("+")),
        };

        match &self.default {
            Some(val) => format!("{} = {}", str, val.to_string()),
            None => str,
        }
    } 
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeConstraint {
    Trait(Ident),
    TypeEnum(Ident),
    LiteralTypeEnum(Vec<SoulType>),
}

impl TypeConstraint {
    pub fn to_string(&self) -> String {
        match self {
            TypeConstraint::Trait(ident) => ident.0.clone(),
            TypeConstraint::TypeEnum(ident) => ident.0.clone(),
            TypeConstraint::LiteralTypeEnum(soul_types) => format!("typeof[{}]", soul_types.iter().map(|ty| ty.to_string()).join(",")),
        }
    }
}























