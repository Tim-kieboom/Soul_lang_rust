use std::io::Result;
use bitflags::bitflags;
use std::collections::BTreeMap;
use super::{generic::Generic, primitive_types::PrimitiveType};
use crate::{meta_data::{convert_soul_error::convert_soul_error::new_soul_error, meta_data::MetaData, type_meta_data::TypeMetaData}, tokenizer::{token::{Token, TokenIterator}, tokenizer::SplitOn}};


bitflags! {
    #[derive(Debug)]
    pub struct TypeModifiers: u8 {
        const DEFAULT = 0b0000_0000;
        const CONST = 0b0000_0001;
        const LITERAL = 0b0000_0010;
        const VOLATILE = 0b0000_0100;
        const STATIC = 0b0000_1000;
    }
}

impl TypeModifiers {
    pub fn from_str(str: &str) -> TypeModifiers {
        match str {
            "const" => TypeModifiers::CONST,
            "Literal" => TypeModifiers::LITERAL,
            "volatile" => TypeModifiers::VOLATILE,
            "static" => TypeModifiers::STATIC,
            _ => TypeModifiers::DEFAULT,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i8)]
pub enum TypeWrappers {
    Invalid = -1,
    ConstRef = 0,
    MutRef = 1,
    Pointer = 2,
    Array = 3,
}

#[derive(Debug)]
pub struct SoulType {
    pub name: String, 
    pub wrappers: Vec<TypeWrappers>,
    pub modifiers: TypeModifiers,
    pub generics: Vec<Generic>,
}

impl SoulType {
    pub fn new(name: String) -> Self {
        SoulType { 
            name, 
            wrappers: Vec::new(), 
            modifiers: TypeModifiers::DEFAULT, 
            generics: Vec::new(), 
        }
    }

    pub fn new_wrappers(name: String, wrappers: Vec<TypeWrappers>) -> Self {
        SoulType { 
            name, 
            wrappers, 
            modifiers: TypeModifiers::DEFAULT, 
            generics: Vec::new(), 
        }
    }

    pub fn from_stringed_type(
        str: &str, 
        token: &Token, 
        type_meta_data: &TypeMetaData, 
        generics: BTreeMap<String, Generic>,
    ) -> Result<SoulType> {
        let tokens = get_type_tokens(str, token);
        
        let mut soul_type = SoulType::new(String::new());

        if tokens.is_empty() {
            return Err(new_soul_error(token, format!("type '{}' is not valid", str).as_str()));
        }

        let mut iterator = TokenIterator::new(tokens);

        loop {
            let modifier = TypeModifiers::from_str(&iterator.current().text);
            if modifier == TypeModifiers::DEFAULT {
                break;
            }

            soul_type.add_modifier(modifier);

            if let None = iterator.next() {
                return Err(new_soul_error(token, "unexpected end while trying to get Type"));
            }
        }

        
    }

    pub fn new_modifiers(name: String, modifiers: TypeModifiers) -> Self {
        SoulType { 
            name, 
            wrappers: Vec::new(), 
            modifiers, 
            generics: Vec::new(), 
        }
    }


    pub fn to_primitive_type(&self, meta_data: &MetaData) -> PrimitiveType {
        PrimitiveType::from_str(&self.name, meta_data)
    }

    pub fn add_modifier(&mut self, modifier: TypeModifiers) {
        self.modifiers |= modifier;
    }

    pub fn is_pointer(&self) -> bool {
        if self.wrappers.is_empty() {
            false
        } 
        else if self.wrappers.len() == 1 {
            self.wrappers[0] == TypeWrappers::Pointer
        } 
        else if self.is_any_ref() {
            self.wrappers[self.wrappers.len() - 2] == TypeWrappers::Pointer
        } 
        else {
            self.wrappers[self.wrappers.len() - 1] == TypeWrappers::Pointer
        }
    }

    pub fn is_array(&self) -> bool {
        if self.wrappers.is_empty() {
            false
        } 
        else if self.wrappers.len() == 1 {
            self.wrappers[0] == TypeWrappers::Array
        } 
        else if self.is_any_ref() {
            self.wrappers[self.wrappers.len() - 2] == TypeWrappers::Array
        } 
        else {
            self.wrappers[self.wrappers.len() - 1] == TypeWrappers::Array
        }
    }

    pub fn is_any_ref(&self) -> bool {
        if self.wrappers.is_empty() {
            false
        } 
        else {
            self.wrappers.last()
                         .is_some_and(|wrap| wrap == &TypeWrappers::ConstRef || wrap == &TypeWrappers::MutRef)
        }
    }
}


fn get_type_tokens(type_name: &str, token: &Token) -> Vec<Token> {
    let parse_tokens = vec![
        " ", "@", "&",
        "[]", "*",
        "const", "static",
        "volatile", "literal",
    ];

    type_name.split_on(&parse_tokens)
             .iter()
             .filter(|str| str != && " " && !str.is_empty())
             .map(|str| Token{text: str.to_string(), line_number: token.line_number, line_offset: token.line_offset})
             .collect()
}













