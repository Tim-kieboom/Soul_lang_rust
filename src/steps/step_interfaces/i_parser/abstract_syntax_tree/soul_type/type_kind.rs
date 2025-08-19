use crate::prelude::*;
use std::collections::HashMap;
use itertools::Itertools;
use my_macros::{CloneWithPool};
use serde::{Deserialize, Serialize};

use crate::{soul_names::{NamesInternalType, NamesTypeModifiers, NamesTypeWrapper, SOUL_NAMES}, steps::step_interfaces::{i_parser::{abstract_syntax_tree::{expression::{Ident, StaticMethode}, soul_type::soul_type::SoulType, spanned::Spanned, staments::{function::{FunctionSignatureRef, LambdaMode, LambdaSignatureRef}, statment::Lifetime}}, scope::SoulPagePath}, i_sementic::sementic_scope::Byte}, utils::serde_multi_ref::MultiRefPool};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum TypeSize {
    Bit8 = 8,
    Bit16 = 16,
    Bit32 = 32,
    Bit64 = 64,
}

impl TypeSize {
    pub fn to_byte(&self) -> Byte {
        (*self).into()
    }
}

impl Into<Byte> for TypeSize {
    fn into(self) -> Byte {
        Byte(self as u8 as u32 / 8)
    }
}

#[derive(Debug, Clone, CloneWithPool, PartialEq, Serialize, Deserialize)]
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
    TypeDefed(Ident),
    Tuple(Vec<SoulType>),
    NamedTuple(HashMap<Ident, SoulType>),
    Function(Box<FunctionSignatureRef>),
    Lambda(LambdaSignatureRef),

    Struct(Ident),
    Class(Ident),
    Trait(Ident),

    // C-style Enums
    Enum(Ident),
    // Rust-style Enums
    Union(Ident),

    UnionVariant(UnionType),
    ExternalType(Spanned<ExternalType>),
    ExternalPath(Spanned<ExternalPath>),

    TypeEnum(Ident, Vec<SoulType>),

    Generic(Ident),
    LifeTime(Ident),

    // for forward delaring for traits
    TraitType(TraitKind),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TraitKind {
    Indexed(Indexed),
    Meth(Meth),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
///name funny
pub struct Meth {
    pub methode: Box<StaticMethode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Indexed {
    pub collection: Box<SoulType>,
    pub index: Box<SoulType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalPath {
    pub name: Ident, 
    pub path: SoulPagePath
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalType {
    pub path: SoulPagePath,
    pub name: Ident,
}
impl ExternalType {
    pub fn to_string(&self) -> String {
        format!("{}::{}", self.path.0, self.name.0)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UntypedKind {
    UntypedInt,
    UntypedUint,
    UntypedFloat,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnionType {
    pub union: UnionKind, 
    pub variant: Ident
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnionKind {
    Union(Ident),
    External(ExternalType),
}

impl UnionType {
    pub fn to_string(&self) -> String {
        match &self.union {
            UnionKind::Union(ident) => format!("{}::{}", ident.0, self.variant.0),
            UnionKind::External(external_type) => format!("{}::{}", external_type.to_string(), self.variant.0),
        }
    }

    pub fn to_union_name_string(&self) -> String {
        match &self.union {
            UnionKind::Union(ident) => ident.0.clone(),
            UnionKind::External(external_type) => external_type.name.0.clone(),
        }
    }
}

impl TypeKind {

    /// if type is untyped go to default type for untyped type (e.g UntypedInt -> Int but i32 -> i32)
    pub fn untyped_to_typed(self) -> Self {
        match self {
            TypeKind::UntypedInt => TypeKind::SystemInt,
            TypeKind::UntypedUint => TypeKind::SystemUint,
            TypeKind::UntypedFloat => TypeKind::Float(TypeSize::Bit32),
            _ => self,
        }
    }
    
    pub fn to_name_string(&self, ref_pool: &MultiRefPool) -> String {
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
            TypeKind::TypeDefed(ident) => ident.0.clone(),
            TypeKind::Tuple(soul_types) => format!("({})", soul_types.iter().map(|ty| ty.to_string(ref_pool)).join(",")),
            TypeKind::NamedTuple(hash_map) => format!("({})", hash_map.iter().map(|(name, ty)| format!("{}: {}", name.0, ty.to_string(ref_pool))).join(",")),
            TypeKind::Function(function_signature) => function_signature.borrow(ref_pool).node.name.0.clone(),
            TypeKind::Struct(ident) => ident.0.clone(),
            TypeKind::Class(ident) => ident.0.clone(),
            TypeKind::Trait(ident) => ident.0.clone(),
            TypeKind::Enum(ident) => ident.0.clone(),
            TypeKind::Union(ident) => ident.0.clone(),
            TypeKind::UnionVariant(union_ty) => format!("{}::{}", union_ty.to_union_name_string(), union_ty.variant.0),
            TypeKind::TypeEnum(ident, ..) => ident.0.clone(),
            TypeKind::Generic(ident) => ident.0.clone(),
            TypeKind::LifeTime(ident) => ident.0.clone(),
            TypeKind::Lambda(signature) => signature.to_type_string(ref_pool),
            TypeKind::ExternalType(Spanned{node: ExternalType{path:_, name}, span:_}) => name.0.clone(),
            TypeKind::ExternalPath(Spanned{node: ExternalPath{path:_, name}, span:_}) => name.0.clone(),
            TypeKind::TraitType(kind) => match kind {
                TraitKind::Indexed(indexed) => format!("TraitType<{}[{}]>", indexed.collection.to_string(ref_pool), indexed.index.to_string(ref_pool)),
                TraitKind::Meth(Meth{methode}) => format!("TraitType<{}.{}()>", methode.callee.node.to_string(ref_pool), methode.name.0)
            },
        }
    }

    pub fn to_string(&self, ref_pool: &MultiRefPool) -> String {
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
            TypeKind::TypeDefed(ident) => ident.0.clone(),
            TypeKind::Tuple(soul_types) => format!("({})", soul_types.iter().map(|ty| ty.to_string(ref_pool)).join(",")),
            TypeKind::NamedTuple(hash_map) => format!("({})", hash_map.iter().map(|(name, ty)| format!("{}: {}", name.0, ty.to_string(ref_pool))).join(",")),
            TypeKind::Function(function_signature) => function_signature.borrow(ref_pool).node.name.0.clone(),
            TypeKind::Struct(ident) => ident.0.clone(),
            TypeKind::Class(ident) => ident.0.clone(),
            TypeKind::Trait(ident) => ident.0.clone(),
            TypeKind::Enum(ident) => ident.0.clone(),
            TypeKind::Union(ident) => ident.0.clone(),
            TypeKind::UnionVariant(union_ty) => union_ty.to_string(),
            TypeKind::TypeEnum(ident, ..) => ident.0.clone(),
            TypeKind::Generic(ident) => ident.0.clone(),
            TypeKind::LifeTime(ident) => ident.0.clone(),
            TypeKind::Lambda(signature) => signature.to_type_string(ref_pool),
            TypeKind::ExternalType(ty) => ty.node.to_string(),
            TypeKind::ExternalPath(path) => format!("<path>{}", path.node.name.0),
            TypeKind::TraitType(kind) => match kind {
                TraitKind::Indexed(indexed) => format!("TraitType<{}[{}]>", indexed.collection.to_string(ref_pool), indexed.index.to_string(ref_pool)),
                TraitKind::Meth(Meth{methode}) => format!("TraitType<{}>", 
                    {
                        let StaticMethode{callee, name, generics, arguments} = methode.as_ref();
                        let generics = if generics.is_empty() {
                            String::new()
                        }
                        else {
                            format!("<{}>", generics.iter().map(|ty| ty.to_string(ref_pool)).join(","))
                        };

                        format!("{}.{}{}({})", callee.node.to_string(ref_pool), name.0, generics, arguments.iter().map(|arg| arg.to_string(ref_pool)).join(","))   
                    }
                ),
            },
        }
    }

    pub fn get_variant(&self, ref_pool: &MultiRefPool) -> &'static str {
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
            TypeKind::TypeDefed(..) => "Custom",
            TypeKind::Tuple(..) => "Tuple",
            TypeKind::NamedTuple(..) => "NamedTuple",
            TypeKind::Function(..) => "Function",
            TypeKind::Struct(..) => "struct",
            TypeKind::Class(..) => "class",
            TypeKind::Trait(..) => "trait",
            TypeKind::Enum(..) => "enum",
            TypeKind::Union(..) => "union",
            TypeKind::UnionVariant{..} => "unionVariant",
            TypeKind::TypeEnum(..) => "typeEnum",
            TypeKind::Generic(..) => "generic",
            TypeKind::LifeTime(..) => "lifetime",
            TypeKind::Lambda(signature) => match signature.borrow(ref_pool).mode {
                LambdaMode::Mut => "MutFn",
                LambdaMode::Const => "ConstFn",
                LambdaMode::Consume => "OnceFn",
            },
            TypeKind::ExternalType(..) => "ExternalType",
            TypeKind::ExternalPath{..} => "ExternalPath",
            TypeKind::TraitType(kind) => match kind  {
                TraitKind::Meth(_) => "meth",
                TraitKind::Indexed(_) => "Indexed",
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Modifier {
    Default,
    Literal,
    Const,
}

impl Modifier {
    pub fn is_mutable(&self) -> bool {
        match self {
            Modifier::Default => true,
            Modifier::Literal |
            Modifier::Const => false,
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














