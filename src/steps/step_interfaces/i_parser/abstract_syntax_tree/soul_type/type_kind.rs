use std::{collections::HashMap, path::{Component, Path, PathBuf}};

use bincode::{Decode, Encode};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{soul_names::{NamesInternalType, SOUL_NAMES}, steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::Ident, function::{FunctionSignature, LambdaMode, LambdaSignature}, soul_type::soul_type::SoulType, spanned::Spanned}};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum TypeSize {
    Bit8,
    Bit16,
    Bit32,
    Bit64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
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
    Unknown(Ident),
    Custom(Ident),
    Tuple(Vec<SoulType>),
    NamedTuple(HashMap<Ident, SoulType>),
    Function(Box<FunctionSignature>),
    Lambda(LambdaSignature),

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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct ExternalPath {
    pub name: Ident, 
    pub path: SoulPagePath,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct ExternalType {
    pub path: SoulPagePath,
    pub name: Ident,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum UntypedKind {
    UntypedInt,
    UntypedUint,
    UntypedFloat,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct UnionType {
    pub union: UnionKind, 
    pub variant: Ident
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum UnionKind {
    Union(Ident),
    External(ExternalType),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Encode, Decode)]
pub struct SoulPagePath(pub String);
impl SoulPagePath {
    pub fn from_path(path: &PathBuf) -> Self {
        let mut soul_path = String::new();
        let mut components = path.components().peekable();

        while let Some(component) = components.next() {
            
            if let Component::Normal(os_str) = component {
                if soul_path.len() > 0 {
                    soul_path.push_str(".");
                }

                if components.peek().is_none() {
                    let path = PathBuf::from(&os_str);
                    let stem = path.file_stem()
                        .map(|s| s.to_string_lossy())
                        .unwrap_or_else(|| os_str.to_string_lossy());
                    
                    soul_path.push_str(&stem);
                } 
                else {
                    soul_path.push_str(&os_str.to_string_lossy());
                }
            }
        }

        Self(soul_path)
    } 

    pub fn to_path_buf(&self, add_soul_extention: bool) -> PathBuf {
        let mut path = PathBuf::new();

        for token in self.0.split(".") {
            path.push(Path::new(token));
        }
        
        if add_soul_extention {
            Self::append_extension(&path, "soul")
        }
        else {
            path
        }
    }

    pub fn get_last_name(&self) -> &str {
        self.0
            .split(".")
            .last()
            .unwrap_or("")
    }

    pub fn pop(&mut self) -> bool {
        if let Some(pos) = self.0.rfind('.') {
            self.0.truncate(pos);
            true
        }
        else {
            false
        }
    }

    fn append_extension(path: &Path, ext: &str) -> PathBuf {
        let mut os_string = path.as_os_str().to_owned();
        os_string.push(".");
        os_string.push(ext);
        PathBuf::from(os_string)
    }
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

impl ExternalType {
    pub fn to_string(&self) -> String {
        format!("{}.{}", self.path.0, self.name.0)
    }
}

impl ExternalPath {
    pub fn to_string(&self) -> String {
        format!("{}.{}", self.path.0, self.name.0)
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
    
    pub fn to_name_string(&self) -> String {
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
            TypeKind::Unknown(ident) => ident.0.clone(),
            TypeKind::Custom(ident) => ident.0.clone(),
            TypeKind::Tuple(soul_types) => format!("({})", soul_types.iter().map(|ty| ty.to_string()).join(", ")),
            TypeKind::NamedTuple(hash_map) => format!("({})", hash_map.iter().map(|(name, ty)| format!("{}: {}", name.0, ty.to_string())).join(", ")),
            TypeKind::Function(function_signature) => function_signature.name.0.clone(),
            TypeKind::Struct(ident) => ident.0.clone(),
            TypeKind::Class(ident) => ident.0.clone(),
            TypeKind::Trait(ident) => ident.0.clone(),
            TypeKind::Enum(ident) => ident.0.clone(),
            TypeKind::Union(ident) => ident.0.clone(),
            TypeKind::UnionVariant(union_ty) => format!("{}::{}", union_ty.to_union_name_string(), union_ty.variant.0),
            TypeKind::TypeEnum(ident, ..) => ident.0.clone(),
            TypeKind::Generic(ident) => ident.0.clone(),
            TypeKind::LifeTime(ident) => ident.0.clone(),
            TypeKind::Lambda(signature) => signature.to_type_string(),
            TypeKind::ExternalType(Spanned{node: ExternalType{path:_, name}, span:_}) => name.0.clone(),
            TypeKind::ExternalPath(Spanned{node: ExternalPath{path:_, name}, span:_}) => name.0.clone(),
        }
    }

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
            TypeKind::Unknown(ident) => format!("Unknown|{}|", ident.0),
            TypeKind::Custom(ident) => ident.0.clone(),
            TypeKind::Tuple(soul_types) => format!("({})", soul_types.iter().map(|ty| ty.to_string()).join(", ")),
            TypeKind::NamedTuple(hash_map) => format!("({})", hash_map.iter().map(|(name, ty)| format!("{}: {}", name.0, ty.to_string())).join(", ")),
            TypeKind::Function(function_signature) => function_signature.name.0.clone(),
            TypeKind::Struct(ident) => ident.0.clone(),
            TypeKind::Class(ident) => ident.0.clone(),
            TypeKind::Trait(ident) => ident.0.clone(),
            TypeKind::Enum(ident) => ident.0.clone(),
            TypeKind::Union(ident) => ident.0.clone(),
            TypeKind::UnionVariant(union_ty) => union_ty.to_string(),
            TypeKind::TypeEnum(ident, ..) => ident.0.clone(),
            TypeKind::Generic(ident) => ident.0.clone(),
            TypeKind::LifeTime(ident) => ident.0.clone(),
            TypeKind::Lambda(signature) => signature.to_type_string(),
            TypeKind::ExternalType(ty) => format!("|ExternalType| {}", ty.node.to_string()),
            TypeKind::ExternalPath(path) => format!("|ExternalPath| {}", path.node.to_string()),
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
            TypeKind::Unknown(..) => "Unknown",
            TypeKind::Custom(..) => "Custom",
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
            TypeKind::Lambda(signature) => match signature.mode {
                LambdaMode::Mut => "MutFn",
                LambdaMode::Const => "ConstFn",
                LambdaMode::Consume => "OnceFn",
            },
            TypeKind::ExternalType(..) => "ExternalType",
            TypeKind::ExternalPath{..} => "ExternalPath",
        }
    }
}



















