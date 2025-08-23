use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{soul_names::{NamesTypeModifiers, NamesTypeWrapper, SOUL_NAMES}, steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::Ident, soul_type::type_kind::TypeKind}};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct  SoulType {
    pub modifier: Modifier,
    pub base: TypeKind,
    pub wrappers: Vec<TypeWrapper>,
    pub generics: Vec<TypeGenericKind>,
}

impl SoulType {
    pub fn none() -> Self {
        Self{ modifier: Modifier::Default, base: TypeKind::None, wrappers: vec![], generics: vec![] }
    } 
    
    pub fn from_type_kind(base: TypeKind) -> Self {
        Self{ modifier: Modifier::Default, base, wrappers: vec![], generics: vec![] }
    }

    pub fn new_unkown<I: Into<Ident>>(name: I) -> Self {
        Self{ modifier: Modifier::Default, base: TypeKind::Unknown(name.into()), wrappers: vec![], generics: vec![] }
    }

    pub fn with_wrappers(mut self, wrapper: Vec<TypeWrapper>) -> Self {
        self.wrappers = wrapper;
        self    
    }

    pub fn with_mod(mut self, modifier: Modifier) -> Self {
        self.modifier = modifier;
        self    
    }

    pub fn is_none_type(&self) -> bool {
        matches!(self.base, TypeKind::None)
    }

    pub fn to_string(&self) -> String {
        if self.generics.is_empty() {
            format!(
                "{} {}{}",
                self.modifier.to_str(),
                self.base.to_string(),
                self.wrappers.iter().map(|wrap| wrap.to_str()).join("")
            )
        }
        else {
            format!(
                "{} {}<{}>{}",
                self.modifier.to_str(),
                self.base.to_string(),
                self.generics.iter().map(|gene| gene.to_string()).join(","),
                self.wrappers.iter().map(|wrap| wrap.to_str()).join("")
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeGenericKind {
    Type(SoulType),
    Lifetime(Lifetime)
}

impl TypeGenericKind {
    pub fn to_string(&self) -> String {
        match self {
            TypeGenericKind::Type(soul_type) => soul_type.to_string(),
            TypeGenericKind::Lifetime(lifetime) => lifetime.name.0.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeWrapper {
    Invalid,
    Array,
    ConstRef(Option<Lifetime>),
    MutRef(Option<Lifetime>),
    Pointer,
    ConstPointer
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Modifier {
    Default,
    Literal,
    Const,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Lifetime{pub name: Ident}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnyRef {
    Invalid,
    ConstRef(Option<Lifetime>),
    MutRef(Option<Lifetime>),
}

impl Modifier {
    pub fn is_mutable(&self) -> bool {
        match self {
            Modifier::Default => true,
            Modifier::Literal |
            Modifier::Const => false,
        }
    }

    pub fn from_str(str: &str) -> Modifier {
        match str {
            val if val == SOUL_NAMES.get_name(NamesTypeModifiers::Constent) => Modifier::Const,
            val if val == SOUL_NAMES.get_name(NamesTypeModifiers::Literal) => Modifier::Literal,
            _ => Modifier::Default
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Modifier::Default => "",
            Modifier::Literal => SOUL_NAMES.get_name(NamesTypeModifiers::Literal),
            Modifier::Const => SOUL_NAMES.get_name(NamesTypeModifiers::Constent),
        }
    }
}

impl TypeWrapper {
    pub fn is_any_ref(&self) -> bool {
        match self {
            TypeWrapper::ConstRef(..) |
            TypeWrapper::MutRef(..) => true,
            _ => false,
        }
    }
    
    pub fn from_str(str: &str) -> TypeWrapper {
        match str {
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::ConstRef) => TypeWrapper::ConstRef(None),
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::MutRef) => TypeWrapper::MutRef(None),
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::Pointer) => TypeWrapper::Pointer,
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::Array)  => TypeWrapper::Array,
            _ => TypeWrapper::Invalid,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            TypeWrapper::Invalid => "<invalid>",
            TypeWrapper::Array => SOUL_NAMES.get_name(NamesTypeWrapper::Array),
            TypeWrapper::ConstRef(..) => SOUL_NAMES.get_name(NamesTypeWrapper::ConstRef),
            TypeWrapper::MutRef(..) => SOUL_NAMES.get_name(NamesTypeWrapper::MutRef),
            TypeWrapper::Pointer => SOUL_NAMES.get_name(NamesTypeWrapper::Pointer),
            TypeWrapper::ConstPointer => " const*",
        }
    }
}

impl AnyRef {
    pub fn from_str(str: &str) -> AnyRef {
        match str {
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::ConstRef) => AnyRef::ConstRef(None),
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::MutRef) => AnyRef::MutRef(None),
            _ => AnyRef::Invalid,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            AnyRef::Invalid => "<invalid>",
            AnyRef::ConstRef(..) => SOUL_NAMES.get_name(NamesTypeWrapper::ConstRef),
            AnyRef::MutRef(..) => SOUL_NAMES.get_name(NamesTypeWrapper::MutRef),
        }
    }
}












