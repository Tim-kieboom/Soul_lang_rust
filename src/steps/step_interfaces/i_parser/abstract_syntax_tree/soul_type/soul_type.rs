use bincode::{Decode, Encode};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use crate::{soul_names::{check_name_allow_types, NamesTypeModifiers, NamesTypeWrapper, SOUL_NAMES}, steps::{parser::literal::get_literal::get_number, step_interfaces::{i_parser::abstract_syntax_tree::{expression::{Expression, Ident}, literal::Literal, pretty_format::ToString, soul_type::type_kind::TypeKind}, i_tokenizer::TokenStream}}};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
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
        let modifier = if self.modifier == Modifier::Default {
            "".into()
        }
        else {
            format!("{} ", self.modifier.to_str())
        };
        
        if self.generics.is_empty() {
            format!(
                "{}{}{}",
                modifier,
                self.base.to_string(),
                self.wrappers.iter().map(|wrap| wrap.to_string()).join("")
            )
        }
        else {
            format!(
                "{}{}<{}>{}",
                modifier,
                self.base.to_string(),
                self.generics.iter().map(|gene| gene.to_string()).join(","),
                self.wrappers.iter().map(|wrap| wrap.to_string()).join("")
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum TypeGenericKind {
    Type(SoulType),
    Expression(Expression),
    Lifetime(Lifetime)
}

impl TypeGenericKind {
    pub fn to_string(&self) -> String {
        match self {
            TypeGenericKind::Type(soul_type) => soul_type.to_string(),
            TypeGenericKind::Expression(expression) => expression.node.to_string(),
            TypeGenericKind::Lifetime(lifetime) => lifetime.name.0.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum TypeWrapper {
    Invalid,
    Array,
    StackArray(u32),
    StackArrayGeneric(SoulType),
    ConstRef(Option<Lifetime>),
    MutRef(Option<Lifetime>),
    Pointer,
    ConstPointer
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum Modifier {
    Default,
    Literal,
    Const,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Encode, Decode)]
pub struct Lifetime{pub name: Ident}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
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
    
    pub fn from_stream(stream: &mut TokenStream) -> TypeWrapper {
        let wrap_i = stream.current_index();

        match stream.current_text().as_str() {
            "[" => (),
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::ConstRef) => return TypeWrapper::ConstRef(None),
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::Pointer) => return TypeWrapper::Pointer,
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::MutRef) => return TypeWrapper::MutRef(None),
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::Array)  => return TypeWrapper::Array,
            _ => return TypeWrapper::Invalid,
        }

        if stream.next().is_none() {
            stream.go_to_index(wrap_i);
            return TypeWrapper::Invalid   
        }

        let stack_array = match get_number(stream.current()) {
            Ok(Literal::Int(num)) => if num < 0 {
                stream.go_to_index(wrap_i);
                return TypeWrapper::Invalid
            }
            else {
                TypeWrapper::StackArray(num as u32)
            },
            Ok(Literal::Uint(num)) => TypeWrapper::StackArray(num as u32),
            _ => {
                if let Err(_) = check_name_allow_types(stream.current_text()) {
                    return TypeWrapper::Invalid
                }
                else {
                    let ty = SoulType::from_type_kind(TypeKind::Unknown(stream.current_text().into()));
                    TypeWrapper::StackArrayGeneric(ty)
                }
            },
        };

        if !stream.peek_is("]") {
            stream.go_to_index(wrap_i);
            return TypeWrapper::Invalid
        }

        if stream.next().is_none() {
            stream.go_to_index(wrap_i);
            return TypeWrapper::Invalid   
        }

        stack_array
    }

    pub fn to_string(&self) -> String {
        match self {
            TypeWrapper::Invalid => "<invalid>".into(),
            TypeWrapper::Array => SOUL_NAMES.get_name(NamesTypeWrapper::Array).into(),
            TypeWrapper::StackArray(num) => format!("[{}]", num),
            TypeWrapper::StackArrayGeneric(ty) => format!("[{}]", ty.to_string()),
            TypeWrapper::ConstRef(..) => SOUL_NAMES.get_name(NamesTypeWrapper::ConstRef).into(),
            TypeWrapper::MutRef(..) => SOUL_NAMES.get_name(NamesTypeWrapper::MutRef).into(),
            TypeWrapper::Pointer => SOUL_NAMES.get_name(NamesTypeWrapper::Pointer).into(),
            TypeWrapper::ConstPointer => " const*".into(),
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
}












