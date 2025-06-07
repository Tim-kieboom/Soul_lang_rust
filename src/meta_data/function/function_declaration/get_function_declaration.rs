use std::collections::BTreeMap;
use crate::meta_data::soul_error::soul_error::{new_soul_error, pass_soul_error, Result, SoulError};
use once_cell::sync::Lazy;

use super::function_declaration::{get_func_names_access_level, FunctionDeclaration};
use crate::{meta_data::{current_context::{current_context::CurrentContext, rulesets::RuleSet}, function::{argument_info::{argument_info::ArgumentInfo, get_arguments::get_arguments}, function_modifiers::FunctionModifiers}, meta_data::MetaData, soul_names::{check_name, NamesInternalType, SOUL_NAMES}, soul_type::{soul_type::SoulType, type_modifiers::TypeModifiers, type_wrappers::TypeWrappers}}, tokenizer::token::TokenIterator};

static LITERAL_STR_ARRAY_STRING: Lazy<String> = Lazy::new(||
    SoulType::from(
            SOUL_NAMES.get_name(NamesInternalType::String).to_string(), 
            vec![TypeWrappers::Array],
            TypeModifiers::Literal,
            vec![],
    ).to_string()
);

pub fn add_function_declaration(
    iter: &mut TokenIterator,
    meta_data: &mut MetaData,
    context: &mut CurrentContext,
    is_forward_declared: bool,
) -> Result<FunctionDeclaration> {
    let begin_index = iter.current_index();

    let result = internal_function_declaration(iter, meta_data, context, is_forward_declared);
    if result.is_err() {
        iter.go_to_index(begin_index); 
    }

    result
}

fn internal_function_declaration(
    iter: &mut TokenIterator,
    meta_data: &mut MetaData,
    context: &mut CurrentContext,
    is_forward_declared: bool,
) -> Result<FunctionDeclaration> {
    let next_id = meta_data
        .scope_store
        .get_mut(&context.current_scope_id)
        .ok_or(new_soul_error(iter.current(), "Internal ewrror: scope not found"))?
        .get_next_function_id();

    let mut function = FunctionDeclaration::new(
        String::new(), 
        None, 
        Vec::new(),
        true,
        next_id,
    );

    loop {
        if iter.next().is_none() {
            return Err(err_get_function_out_of_bounds(iter));
        }

        let modifier = FunctionModifiers::from_str(&iter.current().text);
        if modifier.contains(FunctionModifiers::Default) {
            iter.next_multiple(-1);
            break;
        }

        function.modifiers |= modifier;
    }

    function.name = iter.current().text.clone();
    function.access_level = get_func_names_access_level(&function.name);
    if let Err(err) = check_name(&function.name) {
        return Err(new_soul_error(iter.current(), err.as_str()));
    }

    let mut is_ctor = false;
    if let Some(in_class) = &context.in_class {

        if function.name == "ctor" || function.name == "Ctor" {
            is_ctor = true;
        }

        function.name = format!("{}#{}", in_class.name, function.name);
    }

    if iter.next().is_none() {
        return Err(err_get_function_out_of_bounds(iter));
    }

    if iter.current().text == "<" {
        todo!("generics not yet implemented");
    }

    if iter.current().text != "(" {
        return Err(new_soul_error(iter.current(), format!("function delcaration: '{}' missing '('", function.name).as_str())); 
    }

    let old_index = iter.current_index();
    let arguments = get_arguments(iter, meta_data, context, Some(function.modifiers), &function.name, is_forward_declared)
        .map_err(|err| pass_soul_error(&iter[old_index], format!("while trying to get function declaration: '{}'", function.name).as_str(), err))?;

    if arguments.args.is_empty() && arguments.options.is_empty() {
        if iter.next().is_none() {
            return Err(err_get_function_out_of_bounds(iter));
        }
    }

    if context.rulesets.contains(RuleSet::Const | RuleSet::Literal) {
        todo!("const/Literal functions")
    }

    function.args = arguments.args.clone();
    function.optionals = arguments.options.clone().into_iter()
        .map(|arg| (arg.name.clone(), arg))
        .collect::<BTreeMap<String, ArgumentInfo>>();

    if iter.next().is_none() {
        return Err(err_get_function_out_of_bounds(iter));
    }

    if !is_ctor {
        function.return_type = get_return_type(iter, meta_data, context);
    }
    else {
        function.return_type = Some(context.in_class.clone().unwrap().name);
    }

    if function.name == "main" {

        if let Some(type_name) = &function.return_type {
            if type_name != SOUL_NAMES.get_name(NamesInternalType::Int) {
                return Err(new_soul_error(
                    iter.current(), 
                    format!("function: 'main' can only be on type or type: '{}'", SOUL_NAMES.get_name(NamesInternalType::Int)).as_str()
                ));
            }
        } 
    }

    let possible_function_id = meta_data.try_get_function(
        &function.name, 
        iter, 
        context, 
        &arguments.args, 
        &arguments.options,
        Vec::new(),
    ).ok();

    if let Some(function_id) = possible_function_id {

        if let Some(func_ref) = meta_data.scope_store.get_mut(&function_id.0)
            .ok_or(new_soul_error(iter.current(), "Internal error: scope not found"))?
            .function_store.from_id.get_mut(&function_id.1) 
        {
            
            if !func_ref.is_forward_declared {
                return Err(new_soul_error(
                    iter.current(), 
                    format!(
                        "function with these arguments already exists, name '{}', args: '{}'\n", 
                        function.name, 
                        ArgumentInfo::to_string_slice(&function.args)
                    ).as_str()
                ));
            }

            func_ref.is_forward_declared = false;
        }
    }
    else {
        let old_index = iter.current_index(); 
        meta_data.add_function(iter, context, function.clone())
            .map_err(|err| pass_soul_error(&iter[old_index], "while trying to get function", err))?;
    }

    if function.name == "main" {

        if !function.optionals.is_empty() {
            return Err(new_soul_error(iter.current(), "function 'main' only allows 'main()' and 'main(str[])' as arguments (optionals not allowed remove '= ...')"));
        }

        if function.args.is_empty() {
            return Ok(function);
        }
        else if function.args.len() > 1 {
            return Err(new_soul_error(iter.current(), format!("function 'main' only allows 'main()' and 'main({})' as arguments", LITERAL_STR_ARRAY_STRING.as_str()).as_str()));
        }
        
        if function.args[0].value_type != LITERAL_STR_ARRAY_STRING.as_str() {
            return Err(new_soul_error(iter.current(), format!("function 'main' only allows 'main()' and 'main({})' as arguments", LITERAL_STR_ARRAY_STRING.as_str()).as_str()));
        }
    }

    if let Some(function_overloads) = meta_data.scope_store.get(&context.current_scope_id)
            .ok_or(new_soul_error(iter.current(), "Internal error: scope not found"))?
            .function_store.from_name(&function.name) 
    {

        if function_overloads.first().is_some_and(|func| func.return_type != function.return_type) {
            return Err(new_soul_error(iter.current(), format!("function of same name: '{}' with diffrent returnType already exist you can not overload return types", function.name).as_str()))
        }
    }

    Ok(function)
}

fn get_return_type(iter: &mut TokenIterator, meta_data: &mut MetaData, context: &mut CurrentContext) -> Option<String> {
    SoulType::from_iterator(iter, &meta_data.type_meta_data, &context.current_generics)
        .ok()
        .map(|return_type| return_type.to_string())
}

fn err_get_function_out_of_bounds(iter: &TokenIterator) -> SoulError {
    new_soul_error(iter.current(), "unexpeced end while trying to get function declaration")
}


