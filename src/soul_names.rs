use std::{collections::{HashMap, HashSet}, result};
use enum_iterator::Sequence;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Serialize, Deserialize};

pub static SOUL_NAMES: Lazy<SoulNames> = Lazy::new(|| {
    SoulNames::new()
});

static ILLIGAL_SYMBOOLS: Lazy<HashSet<char>> = Lazy::new(|| HashSet::from([
    '!', '@', '#', '$', 
    '%', '^', '&', '*',
    '(', ')', '-', '+',
    '=', '[', ']', '{',
    '}', '\\', '|', ';',
    '\'', '\"', ',', '.',
    '<', '>', '/', '?', 
    '`', '~',
]));

pub fn check_name_allow_types(name: &str) -> result::Result<(), String> {
  
    if name.starts_with("__") {
        return Err(format!("name: '{}' can not begin wiht '__' ", name));
    }

    if let Some(illigal_name) = SOUL_NAMES.type_less_iligal_names.get(name) {
        return Err(format!("name: '{}' is illigal", illigal_name));
    }

    if let Some(illigal_symbool) = name.chars().find(|ch| ILLIGAL_SYMBOOLS.contains(ch)) {
        return Err(format!("name: '{}', has illigal symbool '{}'", name, illigal_symbool));
    }

    Ok(())
}

pub fn check_name(name: &str) -> result::Result<(), String> {
  
    if name.starts_with("__") {
        return Err(format!("name: '{}' can not begin wiht '__' ", name));
    }

    if let Some(illigal_name) = SOUL_NAMES.iligal_names.get(name) {
        return Err(format!("name: '{}' is illigal", illigal_name));
    }

    if let Some(illigal_symbool) = name.chars().find(|ch| ILLIGAL_SYMBOOLS.contains(ch)) {
        return Err(format!("name: '{}', has illigal symbool '{}'", name, illigal_symbool));
    }

    Ok(())
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoulNames<'a> {
    #[serde(borrow)]
    pub parse_tokens: Vec<&'a str>,
    #[serde(borrow)]
    pub type_modifiers: HashMap<NamesTypeModifiers, &'a str>,
    #[serde(borrow)]
    pub internal_types: HashMap<NamesInternalType, &'a str>,
    #[serde(borrow)]
    pub type_wappers: HashMap<NamesTypeWrapper, &'a str>,
    #[serde(borrow)]
    pub operator_names: HashMap<NamesOperator, &'a str>,
    #[serde(borrow)]
    pub iligal_names: HashSet<&'a str>,
    #[serde(borrow)]
    pub type_less_iligal_names: HashSet<&'a str>,
    #[serde(borrow)]
    pub other_keywords_names: HashMap<NamesOtherKeyWords, &'a str>,
    #[serde(borrow)]
    pub assign_symbools: HashMap<NamesAssignType, &'a str>,
}

impl<'a> SoulNames<'a> {
    fn new() -> Self {
        let type_wappers = HashMap::from([
            (NamesTypeWrapper::ConstRef, "@"),
            (NamesTypeWrapper::MutRef, "&"),
            (NamesTypeWrapper::Pointer, "*"),
            (NamesTypeWrapper::Array, "[]"),
        ]);
        
        let type_modifiers = HashMap::from([
            (NamesTypeModifiers::Literal, "Literal"),
            (NamesTypeModifiers::Constent, "const"),
            (NamesTypeModifiers::Volatile, "volatile"),
            (NamesTypeModifiers::Static, "static"),
        ]);

        let internal_types = HashMap::from([
            (NamesInternalType::Character, "char"),
            (NamesInternalType::Character16, "char16"),
            (NamesInternalType::Character32, "char32"),
            (NamesInternalType::Character64, "char64"),


            (NamesInternalType::Boolean, "bool"),
            (NamesInternalType::String, "str"),
            (NamesInternalType::None, "none"),
            
            (NamesInternalType::UntypedInt, "untypedInt"),
            (NamesInternalType::Int, "int"),
            (NamesInternalType::Int8, "i8"),
            (NamesInternalType::Int16, "i16"),
            (NamesInternalType::Int32, "i32"),
            (NamesInternalType::Int64, "i64"),
            
            (NamesInternalType::UntypedUint, "untypedUint"),
            (NamesInternalType::Uint, "uint"),
            (NamesInternalType::Uint8, "u8"),
            (NamesInternalType::Uint16, "u16"),
            (NamesInternalType::Uint32, "u32"),
            (NamesInternalType::Uint64, "u64"),
            
            (NamesInternalType::UntypedFloat, "untypedFloat"),
            (NamesInternalType::Float8, "f8"),
            (NamesInternalType::Float16, "f16"),
            (NamesInternalType::Float32, "f32"),
            (NamesInternalType::Float64, "f64"),

            (NamesInternalType::Range, "Range"),
            (NamesInternalType::FILE, "FILE"),
        ]);

        let operator_names = HashMap::from([
            (NamesOperator::Increment, "++"),
            (NamesOperator::Decrement, "--"),
            (NamesOperator::Power, "**"),
            (NamesOperator::Root, "</"),
            (NamesOperator::Addition, "+"),
            (NamesOperator::Subtract, "-"),
            (NamesOperator::Multiple, "*"),
            (NamesOperator::Divide, "/"),
            (NamesOperator::Modulo, "%"),
            
            (NamesOperator::IsSmallerEquals, "<="),
            (NamesOperator::IsBiggerEquals, ">="),
            (NamesOperator::NotEquals, "!="),
            (NamesOperator::Equals, "=="),
            (NamesOperator::Not, "!"),
            (NamesOperator::IsSmaller, "<"),
            (NamesOperator::IsBigger, ">"),

            (NamesOperator::Logarithm, "log"),
            (NamesOperator::LogicalOr, "||"),
            (NamesOperator::LogicalAnd, "&&"),
            (NamesOperator::BitWiseOr, "|"),
            (NamesOperator::BitWiseAnd, "&"),
            (NamesOperator::BitWiseXor, "^"),

            (NamesOperator::Range, ".."),
        ]);

        let assign_symbools = HashMap::from([
            (NamesAssignType::Assign, "="),
            (NamesAssignType::AddAssign, "+="),
            (NamesAssignType::SubAssign, "-="),
            (NamesAssignType::MulAssign, "*="),
            (NamesAssignType::DivAssign, "/="),
            (NamesAssignType::ModuloAssign, "%="),
            (NamesAssignType::BitAndAssign, "&="),
            (NamesAssignType::BitOrAssign, "|="),
            (NamesAssignType::BitXorAssign, "^="),

            (NamesAssignType::GetObjectInner, "."),
            (NamesAssignType::Index, "["),
        ]);

        let other_keywords_names = HashMap::from([
            (NamesOtherKeyWords::If, "if"),
            (NamesOtherKeyWords::Else, "else"),
            
            (NamesOtherKeyWords::ForLoop, "for"),
            (NamesOtherKeyWords::InForLoop, "in"),
            (NamesOtherKeyWords::WhileLoop, "while"),
            (NamesOtherKeyWords::BreakLoop, "break"),
            (NamesOtherKeyWords::ContinueLoop, "continue"),
            (NamesOtherKeyWords::Return, "return"),

            (NamesOtherKeyWords::Struct, "struct"),
            (NamesOtherKeyWords::Class, "class"),
            (NamesOtherKeyWords::Union, "union"),
            (NamesOtherKeyWords::TypeEnum, "typeEnum"),
            (NamesOtherKeyWords::Enum, "enum"),

            (NamesOtherKeyWords::SwitchCase, "match"),
            (NamesOtherKeyWords::Typeof, "typeof"),
            (NamesOtherKeyWords::Type, "type"),
            (NamesOtherKeyWords::Trait, "trait"),
            (NamesOtherKeyWords::Impl, "impl"),
            (NamesOtherKeyWords::Where, "where"),
            

            (NamesOtherKeyWords::CopyData, "copy"),
            (NamesOtherKeyWords::Async, "async"),
            (NamesOtherKeyWords::AwaitAsync, "await"),
            (NamesOtherKeyWords::Import, "import"),

            (NamesOtherKeyWords::Use, "use"),
        ]);

        const BASE_TOKENS: &[&str] = &[
            ":=", ",", "[]", "[", "]", 
            "(", ")", "{", "}", ":", "..", 
            ";", "=", "\\", " ", "\t", "\"",
            "\\\"", "::",
        ];

        let mut iligal_names = HashSet::<&str>::new();
        iligal_names.insert(operator_names.get(&NamesOperator::Logarithm).expect("log not impl"));
        iligal_names.extend(type_modifiers.iter().map(|(_, str)| *str));
        iligal_names.extend(other_keywords_names.iter().map(|(_, str)| *str));
        let type_less_iligal_names = iligal_names.clone();
        iligal_names.extend(internal_types.iter().map(|(_, str)| *str));

        let mut parse_tokens: Vec<&str> = BASE_TOKENS.iter().copied().collect();
        parse_tokens.extend(operator_names.iter().filter(|(key, _)| key != &&NamesOperator::Logarithm).map(|(_, str)| *str));
        parse_tokens.extend(assign_symbools.iter().map(|(_, str)| *str));
        parse_tokens.extend(type_wappers.iter().map(|(_, str)| *str));

        //this is so that the tokenizer takes priority over for example '**' over '*'
        parse_tokens = parse_tokens.into_iter()
            .sorted_by(|a, b| Ord::cmp(&a.len(), &b.len()).reverse())
            .collect();

        SoulNames {
            type_wappers,
            parse_tokens, 
            iligal_names,
            type_modifiers, 
            internal_types,
            operator_names,
            assign_symbools,
            other_keywords_names,
            type_less_iligal_names,
        }
    }

    pub fn get_name<T: std::fmt::Debug + SoulNameEnum<'a>>(&self, key: T) -> &'a str {
        key.get_name(self).expect(format!("Internal Error: in SOUL_NAMES.get_name() name: {:?}, is not defined", key).as_str())
    }

    pub fn str_vec_to_regex(vec: &Vec<&str>) -> Regex {
        Regex::new(
            &vec.iter()
                .map(|token| regex::escape(token))
                .collect::<Vec<String>>()
                .join("|")
        ).unwrap()
    }
}

pub trait SoulNameEnum<'a> { 
    fn get_name(&self, key_tokens: &SoulNames<'a>) -> Option<&'a str>; 
}

macro_rules! impl_soul_name_enum {
    ($t:ty, $field:ident) => (
        impl<'a> SoulNameEnum<'a> for $t {
            fn get_name(&self, key_tokens: &SoulNames<'a>) -> Option<&'a str> {
                key_tokens.$field.get(self).map(|v| &**v)
            }
        }
    );
}
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum NamesOtherKeyWords {
    If,
    Else,

    WhileLoop,
    ForLoop,
    InForLoop,
    ContinueLoop,
    BreakLoop,
    Return,

    Struct,
    Class,
    Enum,
    Union,
    TypeEnum,    
    Trait,
    Impl,
    Where,

    SwitchCase,
    Typeof,
    Type,

    CopyData,
    Async,
    AwaitAsync,
    Import,

    Use,
}
impl_soul_name_enum!(NamesOtherKeyWords, other_keywords_names);

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum NamesOperator {
    Not,
    Equals,
    NotEquals,
    IsSmaller,
    IsSmallerEquals,
    IsBigger,
    IsBiggerEquals,

    Addition,
    Increment,
    Decrement,
    Subtract,
    Multiple,
    Divide,
    Modulo,
    Power,
    Root,
    Logarithm,

    BitWiseOr,
    BitWiseAnd,
    BitWiseXor,
    LogicalOr,
    LogicalAnd,

    Range,
}
impl_soul_name_enum!(NamesOperator, operator_names);

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum NamesAssignType {
    Assign,
    VarAssign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModuloAssign,
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,

    GetObjectInner,
    Index
}
impl_soul_name_enum!(NamesAssignType, assign_symbools);

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NamesTypeWrapper {
    ConstRef,
    MutRef,
    Pointer,
    Array,
}
impl_soul_name_enum!(NamesTypeWrapper, type_wappers);

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NamesTypeModifiers{
    Literal,
    Constent,
    Volatile,
    Static
}
impl_soul_name_enum!(NamesTypeModifiers, type_modifiers);


#[derive(Debug, Sequence, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NamesInternalType {
    Character,
    Character16,
    Character32,
    Character64,
    Boolean,
    String,
    None,
    
    UntypedInt,
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    
    UntypedUint,
    Uint,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    
    UntypedFloat,
    Float8,
    Float16,
    Float32,
    Float64,

    Range,
    FILE,
}
impl_soul_name_enum!(NamesInternalType, internal_types);

/// ORDER OF THIS FIRST FEW IN ARRAY MATTERS (should be untypedInt, untypedUint, untypedFl, int, uint, fl, ...)
pub const NAMES_INTERNAL_TYPE_NUMBER: [NamesInternalType; 15] = [
    NamesInternalType::UntypedInt,
    NamesInternalType::UntypedUint,
    NamesInternalType::UntypedFloat,
    
    NamesInternalType::Int,
    NamesInternalType::Uint,
    NamesInternalType::Float32,
    
    NamesInternalType::Int8,
    NamesInternalType::Int16,
    NamesInternalType::Int32,
    NamesInternalType::Int64,

    NamesInternalType::Uint8,
    NamesInternalType::Uint16,
    NamesInternalType::Uint32,
    NamesInternalType::Uint64,

    NamesInternalType::Float64,
];

/// ORDER OF FIRST FEW IN THIS ARRAY MATTERS (should be int, uint, fl, ...)
pub const NAMES_INTERNAL_TYPE_NUMBER_NON_UNTYPED: [NamesInternalType; 12] = [
    NamesInternalType::Int,
    NamesInternalType::Uint,
    NamesInternalType::Float32,

    NamesInternalType::Int8,
    NamesInternalType::Int16,
    NamesInternalType::Int32,
    NamesInternalType::Int64,

    NamesInternalType::Uint8,
    NamesInternalType::Uint16,
    NamesInternalType::Uint32,
    NamesInternalType::Uint64,

    NamesInternalType::Float64,
];

