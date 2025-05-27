use std::{collections::BTreeMap, io::Result};
use crate::{abstract_styntax_tree::abstract_styntax_tree::{BodyNode, IStatment}, meta_data::{convert_soul_error::convert_soul_error::new_soul_error, current_context::current_context::CurrentContext, function::function_declaration::{function_declaration::FunctionDeclaration, get_function_declaration::add_function_declaration}, meta_data::MetaData, scope_and_var::var_info::{VarFlags, VarInfo}, soul_type::soul_type::SoulType}, tokenizer::token::TokenIterator};

use super::{get_body::get_body, multi_stament_result::MultiStamentResult};

pub fn get_function_body(iter: &mut TokenIterator, meta_data: &mut MetaData, context: &mut CurrentContext, open_bracket_stack: &mut usize) -> Result<MultiStamentResult<IStatment>> {
    let function = add_function_declaration(iter, meta_data, context)?;
    
    if iter.next().is_none() {
        return Err(new_soul_error(iter.current(), "unexpected end while trying to get function body"));
    }

    if iter.current().text != "{" {
        return Err(new_soul_error(iter.current(), format!("function body should start with '{}' but starts with '{}'", '{', iter.current().text).as_str()));
    }
    
    if iter.next().is_none() {
        return Err(new_soul_error(iter.current(), "unexpected end while trying to get function body"));
    }

    let arguments = function.args
        .iter()
        .map(|arg| (&arg.name, arg))
        .chain(function.optionals.iter())
        .map(|(name, arg)| {
            let soul_type = SoulType::from_stringed_type(&arg.value_type, iter.current(), &meta_data.type_meta_data, &mut context.current_generics).unwrap();
            let mut var_flags = VarFlags::IsAssigned;
            if soul_type.is_mutable() {
                var_flags |= VarFlags::IsMutable;
            }
            if soul_type.is_literal() {
                var_flags |= VarFlags::IsLiteral;
            }

            (name.clone(), VarInfo::with_var_flag(name.clone(), arg.value_type.clone(), var_flags))
        })
        .collect::<BTreeMap<String, VarInfo>>();

    let function_body = get_body(iter, meta_data, context, Some(arguments), open_bracket_stack)?;

    Ok(MultiStamentResult::new(IStatment::new_function_body(function, function_body)))
}

























