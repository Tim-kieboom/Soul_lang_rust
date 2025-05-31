use std::io::{Error, Result};

use super::statment_type::statment_type::{StatmentIterator, StatmentType};
use crate::{abstract_styntax_tree::{abstract_styntax_tree::{IStatment, IVariable}, get_abstract_syntax_tree::{get_expression::get_function_call::get_function_call::get_function_call, get_function_body::get_function_body, get_stament::{get_assignmet::get_assignment, get_initialize::get_initialize}, multi_stament_result::MultiStamentResult}}, meta_data::{convert_soul_error::convert_soul_error::new_soul_error, current_context::current_context::CurrentContext, meta_data::MetaData, soul_names::check_name}, tokenizer::token::TokenIterator};

pub fn get_statment(iter: &mut TokenIterator, statment_iter: &mut StatmentIterator, meta_data: &mut MetaData, context: &mut CurrentContext) -> Result<MultiStamentResult<IStatment>> {
    let statment_type;
    match statment_iter.next() {
        Some(val) => statment_type = val,
        None => return Err(new_soul_error(iter.current(), "Internal error: StatmentIterator out of range")),
    } 

    if iter.next().is_none() {
        return Err(err_out_of_bounds(iter));
    }

    match statment_type {
        StatmentType::CloseScope => Ok(MultiStamentResult::new(IStatment::CloseScope())),
        StatmentType::EmptyStatment => Ok(MultiStamentResult::new(IStatment::EmptyStatment())),
        StatmentType::Assignment => {
            let variable = get_variable(iter, context, meta_data)?;
            get_assignment(iter, meta_data, context, variable)
                .map(|result| result.assignment)
        },
        StatmentType::Initialize{..} => {
            get_initialize(iter, meta_data, context)
        },
        StatmentType::FunctionBody{..} => {
            get_function_body(iter, statment_iter, meta_data, context)
        }
        StatmentType::FunctionCall => {
            get_function_call(iter, meta_data, context)
                .map(|result_expr| MultiStamentResult::new(IStatment::new_function_call(result_expr.value)))
        },
        StatmentType::Scope => todo!(),
    }
}

fn get_variable(iter: &mut TokenIterator, context: &mut CurrentContext, meta_data: &mut MetaData) -> Result<IVariable> {
    let var_name = iter.current();
    if let Err(msg) = check_name(&var_name.text) {
        return Err(new_soul_error(iter.current(), msg.as_str()));
    }

    let scope = meta_data.scope_store.get(&context.current_scope_id)
        .ok_or(new_soul_error(iter.current(), "Internal Error: could not get scope of: context.current_scope_id"))?;

    let variable = scope.try_get_variable(&var_name.text, &meta_data.scope_store)
        .ok_or(new_soul_error(iter.current(), format!("variable: '{}' could not be found in scope", var_name.text).as_str()))?;

    if iter.next().is_none() {
        return Err(err_out_of_bounds(iter));
    }

    Ok(IVariable::new_variable(&variable.name, &variable.type_name))
}

fn err_out_of_bounds(iter: &TokenIterator) -> Error {
    new_soul_error(iter.current(), "unexpected end while trying to get stament")
}










