use std::collections::BTreeMap;
use itertools::Itertools;

use crate::{meta_data::{class_info::access_level::{AccesLevel}, current_context::current_context::{CurrentGenerics, DefinedGenric}, function::{argument_info::argument_info::ArgumentInfo, function_modifiers::FunctionModifiers}, soul_type::{generic::Generic, soul_type::SoulType}, type_meta_data::{TypeMetaData}}, tokenizer::token::TokenIterator};


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunctionID(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: String,
    pub return_type: Option<String>,
    pub args: Vec<ArgumentInfo>,
    pub optionals: BTreeMap<String, ArgumentInfo>,
    pub generics: BTreeMap<String, Generic>,

    pub modifiers: FunctionModifiers,
    pub is_forward_declared: bool,
    pub id: FunctionID,

    pub access_level: AccesLevel,
}

pub fn get_func_names_access_level(name: &str) -> AccesLevel {
    let is_first_letter_capital = name.chars().next().map_or(false, |ch| ch.is_uppercase());
    if is_first_letter_capital {
        AccesLevel::Public
    }
    else {
        AccesLevel::Private
    }
}

impl FunctionDeclaration {
    pub fn new(
        name: String, 
        return_type: Option<String>, 
        args: Vec<ArgumentInfo>, 
        is_forward_declared: bool,
        id: FunctionID,
    ) -> Self {
        let access_level = get_func_names_access_level(&name);

        FunctionDeclaration { 
            name, 
            return_type, 
            args, 
            optionals: BTreeMap::new(), 
            generics: BTreeMap::new(), 
            modifiers: FunctionModifiers::Default,
            is_forward_declared,
            id,
            access_level,
        }
    }

    pub fn from_optional(
        name: String, 
        return_type: Option<String>, 
        args: Vec<ArgumentInfo>, 
        is_forward_declared: bool,
        id: FunctionID,
        optionals: Vec<ArgumentInfo>,
    ) -> Self {
        let access_level = get_func_names_access_level(&name);

        FunctionDeclaration { 
            name, 
            return_type, 
            args, 
            optionals: optionals.into_iter().map(|op| (op.name.clone(), op)).collect(), 
            generics: BTreeMap::new(), 
            modifiers: FunctionModifiers::Default,
            is_forward_declared,
            id,
            access_level,
        }
    }

    pub fn to_string(&self) -> String {
        let mut string_builder = String::new();
        string_builder.push_str(self.modifiers.to_str());

        string_builder.push_str(&self.name);
        self.generics_into_string(&mut string_builder);
        string_builder.push_str("(");
        self.arguments_into_string(&mut string_builder);
        string_builder.push_str(")");

        if let Some(str) = &self.return_type {
            string_builder.push(' ');
            string_builder.push_str(str);
        }

        string_builder
    }

    pub fn are_arguments_compatible(
        &self, 
        iter: &TokenIterator,
        other_args: &Vec<ArgumentInfo>,
        other_optionals: &Vec<ArgumentInfo>,
        type_meta_data: &TypeMetaData,
        generics: &mut CurrentGenerics,
    ) -> bool {
        if other_args.len() < self.args.len() {
            return false;
        }

        for (other_i, other_arg) in other_args.iter().enumerate() {
            if other_i >= self.args.len() {
                return false;
            }

            let arg = &self.args[other_i];

            self.check_for_implicate_generic(arg, iter, other_arg, type_meta_data, generics);
            if let Err(_) = arg.are_compatible(iter, other_arg, type_meta_data, generics) {
                return false;
            }
        } 

        for other_arg in other_optionals {
            match self.optionals.get(&other_arg.name) {
                Some(arg) => {
                    self.check_for_implicate_generic(arg, iter, other_arg, type_meta_data, generics);
                    if let Err(_) = arg.are_compatible(iter, other_arg, type_meta_data, generics) {
                        return false;
                    }
                },
                None => return false,
            }
            
        }

        true
    }

    fn generics_into_string(&self, string_builder: &mut String) {
        if self.generics.is_empty() {
            return 
        }

        string_builder.push('<');
        for (i, (name, _)) in self.generics.iter().enumerate() {
            string_builder.push_str(name);

            if i != self.generics.len() - 1 {
                string_builder.push_str(", ");
            }
        }

        string_builder.push('>');
    }

    fn arguments_into_string(&self, string_builder: &mut String) {
        if !self.args.is_empty() {
            for arg in &self.args[..self.args.len()-1] {
                string_builder.push_str(&arg.to_string());
                string_builder.push_str(", ");
            }

            let last_arg = self.args.last().unwrap();
            string_builder.push_str(&last_arg.to_string());
        }

        if !self.optionals.is_empty() {
            let optionals = self.optionals.iter()
                .sorted_by(|a, b| Ord::cmp(&a.1.arg_position, &b.1.arg_position))
                .map(|(_name, arg)| arg);
                
            for (index, arg) in optionals.enumerate() {
                string_builder.push_str(&arg.to_string());
                if index != self.optionals.len()-1 {
                    string_builder.push_str(", ");
                }
            }
        }
    }

    fn check_for_implicate_generic(
        &self, 
        arg: &ArgumentInfo,
        iter: &TokenIterator,
        other_arg: &ArgumentInfo,
        type_meta_data: &TypeMetaData,
        generics: &mut CurrentGenerics,
    ) {
        let unchecked_type = SoulType::get_unchecked_from_stringed_type(&arg.value_type, iter.current(), type_meta_data, generics)
            .expect("Internal error: Type not found");

        let arg_type = SoulType::from_stringed_type(&other_arg.value_type, iter.current(), type_meta_data, generics)
            .expect("Internal error: Type not found");

        if let Some(generic) = self.generics.get(&unchecked_type.name) {

            if !generics.is_function_call_defined_generic(&unchecked_type.name) {
                let define_wrappers = unchecked_type.wrappers.into_iter()
                    .filter(|w| !arg_type.wrappers.contains(w))
                    .collect::<Vec<_>>();
                
                let define_modifiers = unchecked_type.modifiers ^ arg_type.modifiers;
                
                let mut define_type = SoulType::new(arg_type.name);
                define_type.wrappers = define_wrappers;
                define_type.modifiers = define_modifiers;
                define_type.generic_defines = unchecked_type.generic_defines;

                let genric_define = DefinedGenric {
                    define_type: define_type.to_string(), 
                    generic: generic.clone()
                };

                generics.function_call_defined_generics
                    .get_or_insert(BTreeMap::new())
                    .insert(unchecked_type.name, genric_define);

            }
        }
    }

}















