use std::collections::HashMap;
use regex::Regex;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoulNames<'a> {
    #[serde(borrow)]
    pub parse_tokens: Vec<&'a str>,
    #[serde(borrow)]
    pub type_modifiers: HashMap<TypeModifiers, &'a str>,
    #[serde(borrow)]
    pub internal_types: HashMap<InternalType, &'a str>,
}

impl<'a> SoulNames<'a> {
    pub fn new() -> Self {
        let type_modifiers = HashMap::from([
            (TypeModifiers::Literal, "Literal"),
            (TypeModifiers::Constent, "const"),
        ]);

        let internal_types = HashMap::from([
            (InternalType::Character, "char"),
            (InternalType::Boolean, "bool"),
            (InternalType::String, "str"),
            (InternalType::None, "none"),
            
            (InternalType::UntypedInt, "untypedInt"),
            (InternalType::Int, "int"),
            (InternalType::Int8, "i8"),
            (InternalType::Int16, "i16"),
            (InternalType::Int32, "i32"),
            (InternalType::Int64, "i64"),
            
            (InternalType::Uint, "untypedUint"),
            (InternalType::Uint, "uint"),
            (InternalType::Uint8, "u8"),
            (InternalType::Uint16, "u16"),
            (InternalType::Uint32, "u32"),
            (InternalType::Uint64, "u64"),
            
            (InternalType::Float32, "untypedFloat"),
            (InternalType::Float32, "f64"),
            (InternalType::Float32, "f32"),
        ]);

        let parse_tokens = vec![
            "</", "**", ":=", "!=", "++", 
            "--", ">=", "<=", "==", "+=", 
            "-=", "/=", "*=", "@", "&", 
            ",", "!", "[]", "[", "]", "(", 
            ")", "{", "}", ":", ";", "<", 
            ">", "+", "-", "/", "*", "=", 
            "\\", " ", "\t",
        ];
        
        SoulNames { 
            parse_tokens, 
            type_modifiers, 
            internal_types,
        }
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
    fn get(&self, key_tokens: &SoulNames<'a>) -> Option<&'a str>; 
}

macro_rules! impl_soul_name_enum {
    ($t:ty, $field:ident) => (
        impl<'a> SoulNameEnum<'a> for $t {
            fn get(&self, key_tokens: &SoulNames<'a>) -> Option<&'a str> {
                if let Some(val) = key_tokens.$field.get(self) {
                    Some(val)
                } 
                else {
                    None
                }
            }
        }
    );
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeModifiers{
    Literal,
    Constent
}
impl_soul_name_enum!(TypeModifiers, type_modifiers);

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InternalType {
    Character,
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
    Float32,
    Float64,
}
impl_soul_name_enum!(InternalType, internal_types);