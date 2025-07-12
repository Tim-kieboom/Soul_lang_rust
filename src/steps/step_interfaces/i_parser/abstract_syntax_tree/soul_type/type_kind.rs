use crate::{soul_names::{NamesTypeModifiers, NamesTypeWrapper, SOUL_NAMES}, steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::Ident, soul_type::soul_type::SoulType, statment::{FunctionSignature, VariableDecl}}};

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
    Function(Box<FunctionSignature>),

    Struct(Ident),
    Class(Ident),
    Interface(Ident),
    Trait(Ident),

    // C-style Enums
    Enum(Ident),
    // Rust-style Enums
    Union(Ident),

    TypeEnum(Ident),

    Generic(Ident)
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
    ConstRef,
    MutRef,
    Pointer
}

impl Modifier {
    pub fn from_str(str: &str) -> Modifier {
        match str {
            val if val == SOUL_NAMES.get_name(NamesTypeModifiers::Constent) => Modifier::Const,
            val if val == SOUL_NAMES.get_name(NamesTypeModifiers::Literal) => Modifier::Literal,
            _ => Modifier::Default
        }
    }
}

impl TypeWrapper {
    pub fn from_str(str: &str) -> TypeWrapper {
        match str {
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::ConstRef) => TypeWrapper::ConstRef,
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::MutRef) => TypeWrapper::MutRef,
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::Pointer) => TypeWrapper::Pointer,
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::Array)  => TypeWrapper::Array,
            _ => TypeWrapper::Invalid,
        }
    }
}















