use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::Ident, statment::{FunctionDecl, FunctionSignature, InterfaceSignature, TraitSignature, VariableDecl}};

pub type TypeSize = u16;

#[derive(Debug, Clone, PartialEq)]
pub enum SoulType {
    Base(TypeKind),
    Modifier{
        modifier: Modifier,
        inner: Box<SoulType>,
    },
    Wrapper{
        wrapper: TypeWrapper,
        inner: Box<SoulType>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    // Primitives
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
    Tuple(Vec<SoulType>),
    Function {
        params: Vec<SoulType>,
        return_type: Box<SoulType>,
    },
    Custom(Ident),

    Struct {
        name: Ident,
        fields: Vec<VariableDecl>,
        implements: Vec<InterfaceSignature>,
    },
    Class {
        name: Ident,
        fields: Vec<VariableDecl>,
        methods: Vec<FunctionDecl>,
        implements: Vec<InterfaceSignature>,
    },
    Interface {
        name: Ident,
        methods: Vec<FunctionSignature>,
    },
    Trait {
        name: Ident,
        methods: Vec<FunctionSignature>,
        requires: Vec<TraitSignature>,
    },

    
    // C-style Enums
    Enum {
        name: Ident,
        variants: Vec<EnumVariant>,
    },
    // Rust-style Enums
    Union {
        name: Ident,
        variants: Vec<UnionVariant>,
    },
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
    Array,
    ConstRef,
    MutRef,
    Pointer
}





