use crate::meta_data::soul_error::soul_error::{new_soul_error, Result};
use crate::{abstract_styntax_tree::abstract_styntax_tree::IStatment, meta_data::{current_context::current_context::CurrentContext, function::function_declaration::get_function_declaration::add_function_declaration, meta_data::MetaData}, tokenizer::token::TokenIterator};

use super::{get_body::get_body, get_stament::statment_type::statment_type::StatmentIterator, multi_stament_result::MultiStamentResult};

pub fn get_function_body(iter: &mut TokenIterator, statment_iter: &mut StatmentIterator, meta_data: &mut MetaData, context: &mut CurrentContext) -> Result<MultiStamentResult<IStatment>> {
    
    let function = add_function_declaration(iter, meta_data, context, false)?;
    
    if iter.current().text != "{" {
        if iter.next().is_none() {
            return Err(new_soul_error(iter.current(), "unexpected end while trying to get function body"));
        }    
    }
    else if iter.current().text != "{" {
        return Err(new_soul_error(iter.current(), format!("function body should start with '{}' but starts with '{}'", '{', iter.current().text).as_str()));
    }
    
    if iter.next().is_none() {
        return Err(new_soul_error(iter.current(), "unexpected end while trying to get function body"));
    }
    
    let function_body = get_body(iter, statment_iter, meta_data, context, Some(function.clone()), false)?;
    if iter.next().is_none() {
        return Err(new_soul_error(iter.current(), "unexpected end while trying to get function body"));
    }
    
    Ok(MultiStamentResult::new(IStatment::new_function_body(function, function_body, iter.current())))
}

























