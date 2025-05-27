use std::io::Result;

use crate::{abstract_styntax_tree::abstract_styntax_tree::IExpression, meta_data::{convert_soul_error::convert_soul_error::new_soul_error, current_context::current_context::CurrentGenerics, soul_names::{NamesTypeModifiers, SOUL_NAMES}, soul_type::soul_type::SoulType, type_meta_data::{TypeMetaData}}, tokenizer::token::TokenIterator};

#[derive(Debug, Clone, PartialEq)]
pub struct ArgumentInfo {
    pub name: String,
    pub value_type: String,

    //if let Some(_) argument is optional 
    pub default_value: Option<IExpression>,

    pub is_mutable: bool,
    pub arg_position: u32,

    pub can_be_multiple: bool,
}

impl ArgumentInfo {
    pub const fn new_argument(
        name: String, 
        value_type: String, 
        is_mutable: bool, 
        arg_position: u32,
    ) -> Self {
        ArgumentInfo { 
            name, 
            value_type, 
            default_value: None, 
            is_mutable, 
            arg_position, 
            can_be_multiple: false, 
        }
    }

    pub const fn new_optional(
        name: String, 
        value_type: String,
        default_value: Option<IExpression>, 
        is_mutable: bool, 
        arg_position: u32,
    ) -> Self {
        ArgumentInfo { 
            name, 
            value_type, 
            default_value, 
            is_mutable, 
            arg_position, 
            can_be_multiple: false, 
        }
    }

    pub const fn new_empty() -> Self {
        ArgumentInfo { 
            name: String::new(), 
            value_type: String::new(), 
            default_value: None, 
            is_mutable: false, 
            arg_position: 0, 
            can_be_multiple: false, 
        }
    }

    pub fn is_optional(&self) -> bool {
        self.default_value.is_some()
    }

    pub fn are_compatible(
        &self, 
        iter: &TokenIterator,
        other: &ArgumentInfo,
        type_meta_data: &TypeMetaData,
        generics: &mut CurrentGenerics,
    ) -> Result<()> {
        
        if self.is_optional() != other.is_optional() {
            return Err(new_soul_error(
                iter.current(), 
                format!("argument not compatible because: arg: '{}' and arg: '{}' one is optional and the other is not", self.to_string(), other.to_string()).as_str()
            ));
        }

        let other_type = SoulType::from_stringed_type(
            &other.value_type, 
            iter.current(), 
            type_meta_data, 
            generics,
        )
            .inspect_err(|err| panic!("Internal Error while trying to run are_compatible other_type from string failed err: {}", err.to_string()))
            .unwrap();
        
        let self_type = SoulType::from_stringed_type(
            &self.value_type, 
            iter.current(), 
            type_meta_data, 
            generics,
        )
            .inspect_err(|err| panic!("Internal Error while trying to run are_compatible self_type from string failed err: {}", err.to_string()))
            .unwrap();

        if !other_type.is_convertable(&self_type, iter.current(), type_meta_data, generics) {
            return Err(new_soul_error(
                iter.current(), 
                format!("argument not compatible because: arg: '{}' and arg: '{}' have diffrent types", self.to_string(), other.to_string()).as_str())
            ); 
        }

        Ok(())
    }

    pub fn to_string(&self) -> String {
        let mut string_builder = String::new();
        self.into_string(&mut string_builder);
        string_builder
    }

    pub fn to_string_slice<'a, I>(args: I) -> String 
    where 
        I: IntoIterator<Item = &'a ArgumentInfo> 
    {
        let mut iter = args.into_iter().peekable();

        if iter.peek().is_none() {
            return "<empty>".to_string();
        }

        let mut string_builder = String::new();
        string_builder.push('{');
        
        for arg in iter {
            string_builder.push_str(", ");
            arg.into_string(&mut string_builder);
        }

        string_builder.push('}');
        string_builder
    }

    pub fn into_string(&self, string_builder: &mut String) {

        if self.is_mutable {
            string_builder.push_str("mut ");
            string_builder.push_str(&self.value_type);
        }
        else {
            let non_const_type = &self.value_type.replacen(
                SOUL_NAMES.get_name(NamesTypeModifiers::Constent), "", 1
            );
            
            string_builder.push_str(non_const_type);
        }

        string_builder.push(' ');
        string_builder.push_str(&self.name);

        if let Some(value) = &self.default_value {
            string_builder.push_str(" = ");
            string_builder.push_str(&value.to_string());
        }  
    }
}


