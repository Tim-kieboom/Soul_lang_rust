use std::result;
use std::collections::HashMap;
use crate::tokenizer::tokenizer::SplitOn;
use crate::meta_data::type_meta_data::TypeMetaData;
use crate::tokenizer::token::{Token, TokenIterator};
use super::primitive_types::{DuckType, NumberCategory};
use crate::meta_data::class_info::class_info::ClassInfo;
use super::type_checker::type_checker::get_primitive_type_from_literal;
use crate::meta_data::current_context::current_context::CurrentGenerics;
use crate::meta_data::soul_names::{NamesInternalType, NamesTypeModifiers, SOUL_NAMES};
use crate::meta_data::soul_error::soul_error::{new_soul_error, pass_soul_error, Result};
use super::{primitive_types::PrimitiveType, type_modifiers::TypeModifiers, type_wrappers::TypeWrappers};


#[derive(Debug, Clone, PartialEq)]
pub struct SoulType {
    pub name: String, 
    pub wrappers: Vec<TypeWrappers>,
    pub modifiers: TypeModifiers,
    pub generic_defines: Vec<SoulType>,
}

impl SoulType {
    pub fn new(name: String) -> Self {
        SoulType { 
            name, 
            wrappers: Vec::new(), 
            modifiers: TypeModifiers::Default, 
            generic_defines: Vec::new(), 
        }
    }

    pub fn new_empty() -> Self {
        SoulType { 
            name: String::new(),
            wrappers: Vec::new(), 
            modifiers: TypeModifiers::Default, 
            generic_defines: Vec::new(),
        }
    }

    pub fn from_modifiers(name: String, modifiers: TypeModifiers) -> Self {
        SoulType { 
            name, 
            wrappers: Vec::new(), 
            modifiers, 
            generic_defines: Vec::new(), 
        }
    }

    pub fn from_wrappers(name: String, wrappers: Vec<TypeWrappers>) -> Self {
        SoulType { 
            name, 
            wrappers, 
            modifiers: TypeModifiers::Default, 
            generic_defines: Vec::new(), 
        }
    }

    pub fn from(name: String, wrappers: Vec<TypeWrappers>, modifiers: TypeModifiers, generic_defines: Vec<SoulType>) -> Self {
        SoulType { 
            name, 
            wrappers, 
            modifiers, 
            generic_defines, 
        }
    }

    pub fn clear(&mut self) {
        self.name.clear();
        self.wrappers.clear();
        self.generic_defines.clear();
        self.modifiers = TypeModifiers::Default;
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_empty() && 
        self.wrappers.is_empty() && 
        self.generic_defines.is_empty() && 
        self.modifiers == TypeModifiers::Default
    }

    pub fn is_class(&self, class_store: &HashMap<String, ClassInfo>) -> bool {
        class_store.contains_key(&self.name)
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn convert_typedef_to_original(&self, token: &Token, type_meta_data: &TypeMetaData, generics: &CurrentGenerics) -> Option<SoulType> {

        if generics.scope_generics.contains_key(&self.name) {
            return Some(self.clone());
        } 
        else if generics.function_call_defined_generics.as_ref().is_some_and(|store| store.contains_key(&self.name)) {
            let define = generics.function_call_defined_generics.as_ref().unwrap().get(&self.name).unwrap();   
            let defined_type = SoulType::from_stringed_type(&define.define_type.clone(), token, type_meta_data, generics)
                .ok()?;

            return defined_type.convert_typedef_to_original(token, type_meta_data, generics);
        }
        
        let id = type_meta_data.type_store.to_id.get(&self.name)?;
        let possible_typedef = type_meta_data.type_store.typedef_store.get(id);
        
        if let Some(typedef) = possible_typedef {
            let type_result = SoulType::from_stringed_type(&typedef.from_stringed, token, type_meta_data, generics);
            if let Err(_) = type_result {
                return None;
            }

            Some(type_result.unwrap())
        } 
        else {
            Some(self.clone())
        }
    }

    pub fn is_convertable(&self, to_type_: &SoulType, token: &Token, type_meta_data: &TypeMetaData, generics: &mut CurrentGenerics) -> bool {
        let possible_from = self.convert_typedef_to_original(token, type_meta_data, generics);
        let possible_to = to_type_.convert_typedef_to_original(token, type_meta_data, generics);

        if possible_from.is_none() || possible_to.is_none() {
            return false;
        }
        let from_type = possible_from.unwrap();
        let to_type = possible_to.unwrap();

        if let Some(generic) = generics.scope_generics.get(&self.name) {
            // if any of T
            if generic.validater.is_none() {
                return true;
            }

            let validater = generic.validater.as_ref().unwrap();
            todo!("generic validater not yet impl")
        }
        else {
            if let Some(is_convertable) = self.is_non_genric_convertable(&from_type, &to_type, type_meta_data) {
                return is_convertable;
            }
        }
        
        if !from_type.are_modifiers_covertable(&to_type) {
            return false;
        }

        if !from_type.are_wrapper_compatible(&to_type) {
            return false;
        }

        if from_type.to_primitive_type(type_meta_data) == PrimitiveType::Object || from_type.is_any_ref() {
            return from_type.name == to_type.name
        }

        true
    }

    fn is_non_genric_convertable(&self, from_type: &SoulType, to_type: &SoulType, type_meta_data: &TypeMetaData) -> Option<bool> {
         if from_type == to_type {
            return Some(true);
        }

        let from_prim_type = from_type.to_primitive_type(type_meta_data);
        let to_prim_type = to_type.to_primitive_type(type_meta_data); 
        if from_prim_type.is_untyped_type() {
            match from_prim_type {
                PrimitiveType::UntypedInt => {
                    if to_prim_type.to_duck_type() != DuckType::Number {
                        return Some(false);
                    }
                },
                PrimitiveType::UntypedUint => {
                    let to_number_category = to_prim_type.to_number_category();
                    if to_number_category != NumberCategory::UnsignedInterger {
                        return Some(false);
                    }
                },
                PrimitiveType::UntypedFloat => {
                    let to_number_category = to_prim_type.to_number_category();
                    if to_number_category != NumberCategory::FloatingPoint {
                        return Some(false);
                    }
                },
                _ => (),
            }
        }

        None
    }

    pub fn are_modifiers_covertable(&self, to_type: &SoulType) -> bool {
        if self.modifiers == to_type.modifiers {
            true
        } 
        else if !self.is_mutable() && to_type.is_mutable() {
            self.is_literal()
        } 
        else {
            true
        }
    }

    pub fn are_wrapper_compatible(&self, to_type: &SoulType) -> bool {
        if self.wrappers.len() != to_type.wrappers.len() {
            return false;
        }

        if self.wrappers.is_empty() {
            return true;
        }

        for i in 0.. self.wrappers.len() {
            if self.wrappers[i] == TypeWrappers::ConstRef && to_type.wrappers[i] == TypeWrappers::MutRef {
                continue;
            }
            
            if self.wrappers[i] != to_type.wrappers[i] {
                return false;
            }
        }

        true
    }

    pub fn is_mutable(&self) -> bool {
        !self.modifiers.contains(TypeModifiers::Literal) &&
        !self.modifiers.contains(TypeModifiers::Const) &&
        self.wrappers.last().is_none_or(|wrap| wrap != &TypeWrappers::ConstRef)        
    }

    pub fn to_string(&self) -> String {
        let mut string_builder = String::new();
        string_builder.push_str(&self.modifiers.to_str());

        string_builder.push_str(&self.name);

        if self.generic_defines.len() > 0 {
            string_builder.push('<');
            for (i, generic) in self.generic_defines.iter().enumerate() {
                string_builder.push_str(&generic.to_string());

                if i+1 < self.generic_defines.len() {
                    string_builder.push(',');
                }
            }

            string_builder.push('>');
        }

        for wrap in self.wrappers.iter() {
            string_builder.push_str(wrap.to_str());
        }
        
        string_builder
    }

    pub fn from_literal<'a>(
        iter: &mut TokenIterator, 
        type_meta_data: &TypeMetaData, 
        generics: &mut CurrentGenerics,
        should_be_type: Option<&SoulType>,
        is_literal: &mut bool,
    ) -> Result<(SoulType, String)> {
        let begin_index = iter.current_index();
        let mut token = iter.current();
        let _token;
        *is_literal = false;

        if iter.current().text == "-" {
            if let None = iter.next() {
                return Err(new_soul_error(iter.current(), "unexpeced end while parsing literal value"));
            }

            _token = Token{
                text: format!("-{}", 
                iter.current().text), 
                line_number: iter.current().line_number, 
                line_offset: iter.current().line_offset,
            };
            token = &_token;
        }

        let value_primitive_type = get_primitive_type_from_literal(&token.text);
        if value_primitive_type != PrimitiveType::Invalid && value_primitive_type != PrimitiveType::Object {
            *is_literal = true;
            let mut soul_type = SoulType::from_modifiers(
                value_primitive_type.to_str().unwrap().to_string(), 
                TypeModifiers::Literal,
            );

            if let Some(should_type) = should_be_type {
                if !soul_type.is_convertable(should_type, token, type_meta_data, generics) {
                    return Err(new_soul_error(token, format!("'{}' and '{}' are not compatible", soul_type.to_string(), should_type.to_string()).as_str()));
                }

                soul_type = should_type.clone();
            }

            return Ok((soul_type, token.text.clone()));
        }

        let possible_tuple = get_literal_array(iter, should_be_type, type_meta_data, generics, is_literal);
        if let Err(err) = possible_tuple {

            let offset = begin_index;
            let size = iter.current_index() - begin_index + 1;
            let mut string_builder = String::new();
            for i in 0..size {
                let current_text = &iter.go_to_index(i + offset).unwrap().text; 
                string_builder.push_str(&current_text);
            }
    
            const SHORTEN_THRESHOLD: usize = 15;
            const SHORTEN_SYMBOOL: &str = "[... ";
            if string_builder.len() > SHORTEN_THRESHOLD {
                let mut short_value = String::with_capacity(SHORTEN_THRESHOLD + SHORTEN_SYMBOOL.len());
                short_value.push_str(SHORTEN_SYMBOOL);
                short_value.push_str(&string_builder[string_builder.len()-SHORTEN_THRESHOLD..]);
                string_builder = short_value;
            }

            iter.go_to_index(begin_index);
            return Err(pass_soul_error(iter.current(), format!("value: '{}' is not valid literal value", string_builder).as_str(), err));
        }
        let tuple = possible_tuple.unwrap();

        let (mut list_type, should_skip_first) = tuple;
        let mut offset = begin_index;
        let mut size = iter.current_index() - begin_index + 1;
        if should_skip_first {
            offset+=1;
            size-=1;
        }

        let mut string_builder = String::new();
        for i in 0..size {
            let current_text = &iter.go_to_index(i + offset).unwrap().text; 
            string_builder.push_str(current_text);
        }

        if let Some(should_type) = should_be_type {
            if !list_type.is_convertable(should_type, iter.current(), type_meta_data, generics) {
                iter.go_to_index(begin_index);
                return Err(new_soul_error(iter.current(), format!("'{}' and '{}' are not compatible", list_type.to_string(), should_type.to_string()).as_str()));
            }

            list_type = should_type.clone();
        }
        
        Ok((list_type, string_builder[1..string_builder.len()-1].to_owned()))
    }

    pub fn try_from_iterator<'a>(
        iter: &mut TokenIterator, 
        type_meta_data: &TypeMetaData, 
        generics: &CurrentGenerics,
        is_wrong_type: &mut bool,
    ) -> Result<SoulType> {
        let begin_index = iter.current_index();

        let result = get_from_iterator(iter, type_meta_data, generics, Some(is_wrong_type));
        if result.is_err() {
            iter.go_to_index(begin_index);
        }
        
        result
    }

    pub fn from_iterator<'a>(
        iter: &mut TokenIterator, 
        type_meta_data: &TypeMetaData, 
        generics: &CurrentGenerics,
    ) -> Result<SoulType> {
        let begin_index = iter.current_index();

        let result = get_from_iterator(iter, type_meta_data, generics, None);
        if result.is_err() {
            iter.go_to_index(begin_index);
        }
        
        result
    }

    pub fn get_unchecked_from_stringed_type(
        str: &str, 
        token: &Token, 
        type_meta_data: &TypeMetaData, 
        generics: &mut CurrentGenerics,
    ) -> Result<SoulType> {
         let tokens = get_type_tokens(str, token);

        let mut soul_type = SoulType::new(String::new());

        if tokens.is_empty() {
            return Err(new_soul_error(token, format!("type '{}' is not valid", str).as_str()));
        }

        let mut iter = TokenIterator::new(tokens);

        loop {
            let modifier = TypeModifiers::from_str(&iter.current().text);
            if modifier == TypeModifiers::Default {
                break;
            }

            soul_type.add_modifier(modifier)
                .map_err(|msg| new_soul_error(iter.current(), format!("while trying to get type\n{}", msg).as_str()))?;

            if let None = iter.next() {
                return Err(new_soul_error(token, "unexpected end while trying to get Type"));
            }
        }

        let type_name = &iter.current().text;

        soul_type.set_name(type_name.clone());

        if iter.peek().is_some_and(|token| token.text == "<") {
            if let None = iter.next() {
                return Err(new_soul_error(token, "unexpected end while parsing type"));
            }

            loop {
                let current_type = SoulType::from_iterator(&mut iter, type_meta_data, generics)?;
                soul_type.generic_defines.push(current_type);

                if let None = iter.next() {
                    return Err(new_soul_error(token, "unexpected end while parsing type"));
                }

                if iter.current().text == ">" {
                    break;
                }
                else if iter.current().text == "," {
                    continue;
                }
                else {
                    return Err(new_soul_error(iter.current(), format!("'{}' is invalid in generic ctor", iter.current().text).as_str()));
                }
            }
        }
        
        while iter.next().is_some() {
            let wrap = TypeWrappers::from_str(&token.text);
            if wrap == TypeWrappers::Invalid {
                return Ok(soul_type);
            }

            if let Err(msg) = soul_type.add_wrapper(wrap) {
                return Err(new_soul_error(iter.current(), msg.as_str()));
            }
        }

        Ok(soul_type)
    }

    pub fn from_stringed_type<'a>(
        str: &str, 
        token: &Token, 
        type_meta_data: &TypeMetaData, 
        generics: &CurrentGenerics,
    ) -> Result<SoulType> {
        SoulType::internal_from_stringed_type(str, token, type_meta_data, generics)
    }

    fn internal_from_stringed_type<'a>(
        str: &str, 
        token: &Token, 
        type_meta_data: &TypeMetaData, 
        generics: &CurrentGenerics,
    ) -> Result<SoulType> {
        let tokens = get_type_tokens(str, token);

        let mut soul_type = SoulType::new(String::new());

        if tokens.is_empty() {
            return Err(new_soul_error(token, format!("type '{}' is not valid", str).as_str()));
        }

        let mut iter = TokenIterator::new(tokens);

        loop {
            let modifier = TypeModifiers::from_str(&iter.current().text);
            if modifier == TypeModifiers::Default {
                break;
            }

            soul_type.add_modifier(modifier)
                .map_err(|msg| new_soul_error(iter.current(), format!("while trying to get type\n{}", msg).as_str()))?;

            if let None = iter.next() {
                return Err(new_soul_error(token, "unexpected end while trying to get Type"));
            }
        }

        let type_name = &iter.current().text;

        let is_type = type_meta_data.type_store.to_id.contains_key(type_name);
        let is_template = generics.scope_generics.contains_key(type_name) || generics.function_call_defined_generics.as_ref().is_some_and(|store| store.contains_key(&iter.current().text));

        if !is_type && !is_template {
            return Err(new_soul_error(token, format!("'{}' is not reconized type", type_name).as_str()));
        }

        soul_type.set_name(type_name.clone());

        if iter.peek().is_some_and(|token| token.text == "<") {
            if let None = iter.next() {
                return Err(new_soul_error(token, "unexpected end while parsing type"));
            }

            loop {
                let current_type = SoulType::from_iterator(&mut iter, type_meta_data, generics)?;
                soul_type.generic_defines.push(current_type);

                if let None = iter.next() {
                    return Err(new_soul_error(token, "unexpected end while parsing type"));
                }

                if iter.current().text == ">" {
                    break;
                }
                else if iter.current().text == "," {
                    continue;
                }
                else {
                    return Err(new_soul_error(iter.current(), format!("'{}' is invalid in generic ctor", iter.current().text).as_str()));
                }
            }
        }
        
        while iter.next().is_some() {
            let wrap = TypeWrappers::from_str(&iter.current().text);
            if wrap == TypeWrappers::Invalid {
                return Ok(soul_type);
            }

            if let Err(msg) = soul_type.add_wrapper(wrap) {
                return Err(new_soul_error(iter.current(), msg.as_str()));
            }
        }

        Ok(soul_type)
    }

    #[inline(always)]
    pub fn to_primitive_type(&self, type_meta_data: &TypeMetaData) -> PrimitiveType {
        PrimitiveType::from_str(&self.name, type_meta_data)
    }

    #[inline(always)]
    pub fn is_untyped_type(&self, type_meta_data: &TypeMetaData) -> bool {
        self.to_primitive_type(type_meta_data).is_untyped_type()
    } 

    #[inline(always)]
    pub fn to_class_info<'a>(&self, meta_data: &'a TypeMetaData) -> Option<&'a ClassInfo> {
        meta_data.class_store.get(&self.name)
    }

    pub fn add_modifier(&mut self, modifier: TypeModifiers) -> result::Result<(), String> {
        self.modifiers |= modifier;
        
        if self.modifiers.contains(TypeModifiers::Literal) && 
           self.modifiers.contains(TypeModifiers::Const)
        {
            Err(format!("can not have {} and {} in the same type", SOUL_NAMES.get_name(NamesTypeModifiers::Literal), SOUL_NAMES.get_name(NamesTypeModifiers::Constent)))
        }
        else {
            Ok(())
        }
    }

    #[inline(always)]
    pub fn remove_modifier(&mut self, modifier: TypeModifiers) {
        self.modifiers.remove(modifier);
    }

    pub fn add_wrapper(&mut self, wrap: TypeWrappers) -> result::Result<(), String> {
        let last_wrap = self.wrappers.last().cloned().unwrap_or(TypeWrappers::Invalid);
        
        //i32&* and i32&[] are illigal
        if last_wrap.is_any_ref() && !wrap.is_any_ref() {
            return Err(format!("can not have typeWrapper'{:?}' after a refrence", wrap));
        }

        self.wrappers.push(wrap);
        Ok(())
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

    #[inline(always)]
    pub fn is_literal(&self) -> bool {
        self.modifiers.contains(TypeModifiers::Literal)
    }

    #[inline(always)]
    pub fn is_const(&self) -> bool {
        self.modifiers.contains(TypeModifiers::Const)
    }

    #[inline(always)]
    pub fn is_any_ref(&self) -> bool {
        if self.wrappers.is_empty() {
            false
        } 
        else {
            self.wrappers.last()
                         .is_some_and(|wrap| wrap == &TypeWrappers::ConstRef || wrap == &TypeWrappers::MutRef)
        }
    }

    #[inline(always)]
    pub fn is_mut_ref(&self) -> bool {
        if self.wrappers.is_empty() {
            false
        } 
        else {
            self.wrappers.last()
                         .is_some_and(|wrap| wrap == &TypeWrappers::MutRef)
        }
    }

    #[inline(always)]
    pub fn is_const_ref(&self) -> bool {
        if self.wrappers.is_empty() {
            false
        } 
        else {
            self.wrappers.last()
                         .is_some_and(|wrap| wrap == &TypeWrappers::ConstRef)
        }
    }

    #[inline(always)]
    pub fn get_type_child(&self) -> Option<SoulType> {
        if self.wrappers.is_empty() {
            None
        }
        else {
            let mut new_type = self.clone();
            new_type.wrappers.pop();
            Some(new_type)
        }
    }
}

fn get_literal_array(
    iter: &mut TokenIterator, 
    should_be_type: Option<&SoulType>,
    type_meta_data: &TypeMetaData, 
    generics: &mut CurrentGenerics,
    is_literal: &mut bool,
) -> Result<(SoulType, bool)> { 
    const ARRAY_START: &str = "[";
    const ARRAY_END: &str = "]";
    
    let mut skip_first_token = false;
    let mut has_soul_type = false;

    let mut result_type = SoulType::new(SOUL_NAMES.get_name(NamesInternalType::None).to_string());
    let possible_cast_type = SoulType::from_iterator(iter, type_meta_data, generics);
    if let Ok(mut cast_type) = possible_cast_type {
        
        if let Err(msg) = cast_type.add_wrapper(TypeWrappers::Array) {
            return Err(new_soul_error(
                iter.current(), 
                format!("while trying to get literal array\n{}", msg).as_str()
            ));
        }

        result_type = cast_type;
        if !result_type.is_literal() {
            result_type.add_modifier(TypeModifiers::Literal)
                .map_err(|msg| new_soul_error(iter.current(), format!("while trying to get literal array\n{}", msg).as_str()))?;
        }

        has_soul_type = true;
        skip_first_token = true;
        if let None = iter.next() {
            return Err(new_soul_error(iter.current(), "unexpeted end while parsing literal value"));
        }
    }
    
    if iter.current().text != ARRAY_START {
        return Err(new_soul_error(iter.current(), format!("Literal array should start with '{}'", ARRAY_START).as_str()));
    }

    if iter.peek().is_some_and(|token| token.text == ARRAY_END) {
        return Ok((result_type, skip_first_token));
    } 

    *is_literal = true;

    let mut should_be_type_element: Option<&SoulType> = None;
    let _should_be_type_element;
    if let Some(should_type) = should_be_type{
        _should_be_type_element = should_type.get_type_child();
        should_be_type_element = _should_be_type_element.as_ref();
    }

    while let Some(_) = iter.next() {
        if iter.current().text == ARRAY_END {
            if result_type.name.is_empty() && should_be_type.is_some() {
                result_type = should_be_type.unwrap().clone();
            }
            return Ok((result_type, skip_first_token));
        }

        let mut dummy = false;
        let mut element_type;
        match SoulType::from_literal(iter, type_meta_data, generics, should_be_type_element, &mut dummy) {
            Ok(val) => element_type = val.0,
            Err(err) => {
                
                let c_string = type_meta_data.c_str_store.from_name(&iter.current().text);
                if let None = c_string {
                    return Err(pass_soul_error(iter.current(), format!("while trying to get literal array").as_str(), err))
                }

                element_type = SoulType::from_modifiers(
                    SOUL_NAMES.get_name(NamesInternalType::String).to_string(), 
                    TypeModifiers::Literal,
                );
            },
        }

        if element_type.to_primitive_type(type_meta_data) == PrimitiveType::UntypedFloat &&
           result_type.to_primitive_type(type_meta_data).is_untyped_type() 
        {
            result_type.name = PrimitiveType::UntypedFloat.to_str().expect("Internal error: UntypedFloat.to_str() NotImpl").to_string()
        }

        if let Err(err_msg) = element_type.add_wrapper(TypeWrappers::Array) {
            return Err(new_soul_error(iter.current(), format!("while trying to get literal array\n {}", err_msg).as_str()))
        }

        if has_soul_type {
            if !element_type.is_convertable(&result_type, iter.current(), type_meta_data, generics) {
                return Err(new_soul_error(
                    iter.current(), 
                    format!(
                        "type '{}' and type '{}' can not be in the same array", 
                        get_element_type_string(&element_type), 
                        get_element_type_string(&result_type)
                    ).as_str()
                ));
            }
        }
        else {
            result_type = element_type;
            has_soul_type = true;
        }

        if let None = iter.next() {
            break;
        }

        if iter.current().text == ARRAY_END {
            return Ok((result_type, skip_first_token));
        }
        else if iter.current().text != "," {
            return Err(new_soul_error(iter.current(), format!("token '{}' is not allowed in literal array", iter.current().text).as_str()));
        }
    }
    
    Err(new_soul_error(iter.current(), "unexpeced end while parsing literal array"))
}

fn get_element_type_string(array_type: &SoulType) -> String {
    array_type.get_type_child()
        .as_ref()
        .unwrap_or(array_type)
        .to_string()
}

fn get_from_iterator(
    iter: &mut TokenIterator, 
    type_meta_data: &TypeMetaData, 
    generics: &CurrentGenerics,
    mut is_wrong_type: Option<&mut bool>
) -> Result<SoulType> { 
    
    let mut soul_type = SoulType::new(String::new());

    loop {
        let mod_type = TypeModifiers::from_str(&iter.current().text);
        if mod_type == TypeModifiers::Default {
            break;
        }

        soul_type.add_modifier(mod_type)
            .map_err(|msg| new_soul_error(iter.current(), format!("while trying to get type\n{}", msg).as_str()))?;

        if let None = iter.next() {
            return Err(new_soul_error(&iter.current(), "unexpected end while trying to get Type"));
        }
    }

    if !generics.scope_generics.contains_key(&iter.current().text) {

        if let None = type_meta_data.get_type_id(&iter.current().text) {
            return Err(new_soul_error(&iter.current(), format!("'{}' is not reconized type", iter.current().text).as_str()));
        }
    }


    soul_type.set_name(iter.current().text.clone());

    if iter.peek().is_some_and(|token| token.text == "<") {
        if let None = iter.next() {
            return Err(new_soul_error(iter.current(), "unexpected end while trying to get Type"));
        }

        let generic_defines = get_defined_generic(iter, type_meta_data, &generics, &mut soul_type)
            .map_err(|err| {
                if let Some(wrong_type) = &mut is_wrong_type {
                    **wrong_type = true;
                } 
                err
            })?;

        soul_type.generic_defines = generic_defines;
    }

    if let Some(class) = soul_type.to_class_info(type_meta_data) {
        let msg = format!("in type: '{}', amount of template types defined: '{}' does not equal the amount of template types in class: '{}'", soul_type.name, soul_type.generic_defines.len(), class.generics.len());
        return Err(new_soul_error(iter.current(), msg.as_str())); 
    }

    while let Some(token) = iter.next() {
        let wrap = TypeWrappers::from_str(&token.text);
        if wrap == TypeWrappers::Invalid {

            if let None = iter.next_multiple(-1) {
                return Err(new_soul_error(iter.current(), "unexpected end while trying to get Type"));
            }

            return Ok(soul_type);
        }

        if let Err(msg) = soul_type.add_wrapper(wrap) {
            if let Some(wrong_type) = is_wrong_type {
                *wrong_type = true;
            }

            return Err(new_soul_error(iter.current(), msg.as_str()));
        }

    }

    Ok(soul_type)
}

fn get_defined_generic<'a>(
    iter: &mut TokenIterator, 
    type_meta_data: &TypeMetaData, 
    generics: &CurrentGenerics, 
    soul_type: &mut SoulType,
) -> Result<Vec<SoulType>> {
    let mut generics_defs = Vec::new();
    let possible_class = soul_type.to_class_info(type_meta_data);
    if let None = possible_class {
        return Err(new_soul_error(iter.current(), format!("you are defining template type (<TYPE>) but parent type: '{}' is not a class", soul_type.name).as_str()));
    }
    let class = possible_class.unwrap();

    loop {
        if iter.next().is_none() {
            return Err(new_soul_error(iter.current(), "unexpected end while trying to get Type"));
        }

        let generic_type = SoulType::from_iterator(iter, type_meta_data, generics)?;
        generics_defs.push(generic_type);
        if iter.next().is_none() {
            return Err(new_soul_error(iter.current(), "unexpected end while trying to get Type"));
        }

        if iter.current().text == ">" {
            break;
        }
        else if iter.current().text == "," {
            continue;
        }
        else {
            return Err(new_soul_error(iter.current(), format!("'{}' is invalid in template type ctor", iter.current().text).as_str()));
        }
    }

    if class.generics.len() != generics_defs.len() {
        let msg = format!("in type: '{}', amount of template types defined: '{}' does not equal the amount of template types in class: '{}'", soul_type.name, generics_defs.len(), class.generics.len());
        return Err(new_soul_error(iter.current(), msg.as_str()));
    }

    Ok(generics_defs)
}

fn get_type_tokens(type_name: &str, token: &Token) -> Vec<Token> {
    let mut parse_tokens = SOUL_NAMES.type_wappers.iter().map(|(_, name)| *name).collect::<Vec<_>>();
    parse_tokens.extend(SOUL_NAMES.type_modifiers.iter().map(|(_, name)| *name));
    parse_tokens.push(" ");

    type_name.split_on(&parse_tokens)
             .iter()
             .filter(|str| str != && " " && !str.is_empty())
             .map(|str| Token{text: str.to_string(), line_number: token.line_number, line_offset: token.line_offset})
             .collect()            
}









