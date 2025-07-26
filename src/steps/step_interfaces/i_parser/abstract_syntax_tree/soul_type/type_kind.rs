use std::collections::HashMap;

use itertools::Itertools;

use crate::{soul_names::{NamesInternalType, NamesTypeModifiers, NamesTypeWrapper, SOUL_NAMES}, steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::Ident, soul_type::soul_type::SoulType, staments::{function::FunctionSignatureRef, statment::{Lifetime, VariableDecl}}}};

#[derive(Debug, Clone, PartialEq)]
pub enum TypeSize {
    Bit8,
    Bit16,
    Bit32,
    Bit64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
// Primitives
    None,
    UntypedInt,
    SystemInt,
    Int(TypeSize),
    SystemUint,
    UntypedUint,
    Uint(TypeSize),
    UntypedFloat,
    Float(TypeSize),
    Char(TypeSize),
    Bool,
    Str,

// complex
    Custom(Ident),
    Tuple(Vec<SoulType>),
    NamedTuple(HashMap<Ident, SoulType>),
    Function(Box<FunctionSignatureRef>),

    Struct(Ident),
    Class(Ident),
    Trait(Ident),

    // C-style Enums
    Enum(Ident),
    // Rust-style Enums
    Union(Ident),

    TypeEnum(Ident, Vec<SoulType>),

    Generic(Ident),
    LifeTime(Ident),
}

impl TypeKind {
    pub fn to_string(&self) -> String {
        match self {
            TypeKind::None => SOUL_NAMES.get_name(NamesInternalType::None).to_string(),
            TypeKind::UntypedInt => SOUL_NAMES.get_name(NamesInternalType::UntypedInt).to_string(),
            TypeKind::SystemInt => SOUL_NAMES.get_name(NamesInternalType::Int).to_string(),
            TypeKind::Int(type_size) => {
                        match type_size {
                            TypeSize::Bit8 => SOUL_NAMES.get_name(NamesInternalType::Int8).to_string(),
                            TypeSize::Bit16 => SOUL_NAMES.get_name(NamesInternalType::Int16).to_string(),
                            TypeSize::Bit32 => SOUL_NAMES.get_name(NamesInternalType::Int32).to_string(),
                            TypeSize::Bit64 => SOUL_NAMES.get_name(NamesInternalType::Int64).to_string(),
                        }
                    },
            TypeKind::SystemUint => SOUL_NAMES.get_name(NamesInternalType::Uint).to_string(),
            TypeKind::UntypedUint => SOUL_NAMES.get_name(NamesInternalType::UntypedUint).to_string(),
            TypeKind::Uint(type_size) => {
                        match type_size {
                            TypeSize::Bit8 => SOUL_NAMES.get_name(NamesInternalType::Uint8).to_string(),
                            TypeSize::Bit16 => SOUL_NAMES.get_name(NamesInternalType::Uint16).to_string(),
                            TypeSize::Bit32 => SOUL_NAMES.get_name(NamesInternalType::Uint32).to_string(),
                            TypeSize::Bit64 => SOUL_NAMES.get_name(NamesInternalType::Uint64).to_string(),
                        }
                    },
            TypeKind::UntypedFloat => SOUL_NAMES.get_name(NamesInternalType::UntypedFloat).to_string(),
            TypeKind::Float(type_size) => {
                        match type_size {
                            TypeSize::Bit8 => SOUL_NAMES.get_name(NamesInternalType::Float8).to_string(),
                            TypeSize::Bit16 => SOUL_NAMES.get_name(NamesInternalType::Float16).to_string(),
                            TypeSize::Bit32 => SOUL_NAMES.get_name(NamesInternalType::Float32).to_string(),
                            TypeSize::Bit64 => SOUL_NAMES.get_name(NamesInternalType::Float64).to_string(),
                        }
                    },
            TypeKind::Char(type_size) => {
                        match type_size {
                            TypeSize::Bit8 => SOUL_NAMES.get_name(NamesInternalType::Uint8).to_string(),
                            TypeSize::Bit16 => SOUL_NAMES.get_name(NamesInternalType::Uint16).to_string(),
                            TypeSize::Bit32 => SOUL_NAMES.get_name(NamesInternalType::Uint32).to_string(),
                            TypeSize::Bit64 => SOUL_NAMES.get_name(NamesInternalType::Uint64).to_string(),
                        }
                    },
            TypeKind::Bool => SOUL_NAMES.get_name(NamesInternalType::Boolean).to_string(),
            TypeKind::Str => SOUL_NAMES.get_name(NamesInternalType::String).to_string(),
            TypeKind::Custom(ident) => ident.0.clone(),
            TypeKind::Tuple(soul_types) => format!("({})", soul_types.iter().map(|ty| ty.to_string()).join(",")),
            TypeKind::NamedTuple(hash_map) => format!("({})", hash_map.iter().map(|(name, ty)| format!("{}: {}", name.0, ty.to_string())).join(",")),
            TypeKind::Function(function_signature) => function_signature.borrow().name.0.clone(),
            TypeKind::Struct(ident) => ident.0.clone(),
            TypeKind::Class(ident) => ident.0.clone(),
            TypeKind::Trait(ident) => ident.0.clone(),
            TypeKind::Enum(ident) => ident.0.clone(),
            TypeKind::Union(ident) => ident.0.clone(),
            TypeKind::TypeEnum(ident, ..) => ident.0.clone(),
            TypeKind::Generic(ident) => ident.0.clone(),
            TypeKind::LifeTime(ident) => ident.0.clone(),
        }
    }

    pub fn get_variant(&self) -> &'static str {
        match self {
            TypeKind::None => "none",
            TypeKind::UntypedInt => "untypedInt",
            TypeKind::SystemInt => "int",
            TypeKind::Int(type_size) => match type_size {
                TypeSize::Bit8 => "i8",
                TypeSize::Bit16 => "i16",
                TypeSize::Bit32 => "i32",
                TypeSize::Bit64 => "i64",
            },
            TypeKind::SystemUint => "uint",
            TypeKind::UntypedUint => "untypedUint",
            TypeKind::Uint(type_size) => match type_size {
                TypeSize::Bit8 => "u8",
                TypeSize::Bit16 => "u16",
                TypeSize::Bit32 => "u32",
                TypeSize::Bit64 => "u64",
            },
            TypeKind::UntypedFloat => "untypedFloat",
            TypeKind::Float(type_size) => match type_size {
                TypeSize::Bit8 => "f8",
                TypeSize::Bit16 => "f16",
                TypeSize::Bit32 => "f32",
                TypeSize::Bit64 => "f64",
            },
            TypeKind::Char(type_size) => match type_size {
                TypeSize::Bit8 => "char",
                TypeSize::Bit16 => "char16",
                TypeSize::Bit32 => "char32",
                TypeSize::Bit64 => "char64",
            },
            TypeKind::Bool => "bool",
            TypeKind::Str => "str",
            TypeKind::Custom(..) => "Custom",
            TypeKind::Tuple(..) => "Tuple",
            TypeKind::NamedTuple(..) => "NamedTuple",
            TypeKind::Function(..) => "Function",
            TypeKind::Struct(..) => "struct",
            TypeKind::Class(..) => "class",
            TypeKind::Trait(..) => "trait",
            TypeKind::Enum(..) => "enum",
            TypeKind::Union(..) => "union",
            TypeKind::TypeEnum(..) => "typeEnum",
            TypeKind::Generic(..) => "generic",
            TypeKind::LifeTime(..) => "lifetime",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub name: Ident,
    pub value: Option<u64>, // Optional manual value
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionVariant {
    pub name: Ident,
    pub fields: Vec<UnionVariantKind>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnionVariantKind {
    Unit,
    Tuple(Vec<SoulType>),
    Struct(Vec<VariableDecl>)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Modifier {
    Default,
    Literal,
    Const,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeWrapper {
    Invalid,
    Array,
    ConstRef(Option<Lifetime>),
    MutRef(Option<Lifetime>),
    Pointer,
    ConstPointer
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnyRef {
    Invalid,
    ConstRef(Option<Lifetime>),
    MutRef(Option<Lifetime>),
}

impl TypeWrapper {
    pub fn is_any_ref(&self) -> bool {
        match self {
            TypeWrapper::ConstRef(..) |
            TypeWrapper::MutRef(..) => true,
            _ => false,
        }
    }
}

impl Modifier {
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














