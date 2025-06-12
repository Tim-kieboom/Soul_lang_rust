use crate::{abstract_styntax_tree::get_abstract_syntax_tree::get_body::get_body, meta_data::{soul_error::soul_error::{new_soul_error, Result, SoulError}, soul_names::NamesInternalType}};
use super::statment_type::statment_type::{StatmentIterator, StatmentType};
use crate::{abstract_styntax_tree::{abstract_styntax_tree::{IStatment, IVariable}, get_abstract_syntax_tree::{get_expression::{get_expression::get_expression, get_function_call::get_function_call::get_function_call}, get_function_body::get_function_body, get_stament::{get_assignmet::get_assignment, get_initialize::get_initialize}, multi_stament_result::MultiStamentResult}}, meta_data::{current_context::current_context::CurrentContext, meta_data::MetaData, soul_names::{check_name, NamesOtherKeyWords, SOUL_NAMES}, soul_type::soul_type::SoulType}, tokenizer::token::TokenIterator};

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
        StatmentType::CloseScope{..} => Ok(MultiStamentResult::new(IStatment::CloseScope())),
        StatmentType::EmptyStatment => Ok(MultiStamentResult::new(IStatment::EmptyStatment())),
        StatmentType::Assignment => {
                const IS_INITIALIZE: bool = false;
                let variable = get_variable(iter, context, meta_data)?;
                get_assignment(iter, meta_data, context, variable, IS_INITIALIZE)
                    .map(|result| result.assignment)
            },
        StatmentType::Initialize{..} => get_initialize(iter, meta_data, context),
        StatmentType::FunctionBody{..} => get_function_body(iter, statment_iter, meta_data, context),
        StatmentType::FunctionCall => {
                let result = get_function_call(iter, meta_data, context)
                    .map(|result_expr| MultiStamentResult::new(IStatment::new_function_call(result_expr.value, iter.current())));
    
                if iter.next().is_none() {
                    return Err(err_out_of_bounds(iter));
                }
                result
            },
        StatmentType::Scope{..} => todo!(),
        StatmentType::Return => get_return(iter, context, meta_data),
        StatmentType::If{..} => get_if_statment(iter, statment_iter, context, meta_data),
        StatmentType::Else{..} => get_else_statment(iter, statment_iter, context, meta_data),
        StatmentType::ElseIf{..} => get_else_if_statment(iter, statment_iter, context, meta_data),
    }
}

fn get_else_if_statment(iter: &mut TokenIterator, statment_iter: &mut StatmentIterator, context: &mut CurrentContext, meta_data: &mut MetaData) -> Result<MultiStamentResult<IStatment>> {
    let begin_i = iter.current_index();
    if iter.current().text != SOUL_NAMES.get_name(NamesOtherKeyWords::Else) {
        return Err(new_soul_error(iter.current(), format!("Internal error: else if Statment doesn't start with '{}'", SOUL_NAMES.get_name(NamesOtherKeyWords::Else)).as_str()));
    }
    if context.current_function.is_none() {
        return Err(new_soul_error(iter.current(), "trying to use else if statment while not being in function"));
    }

    if iter.next().is_none() {
        return Err(err_out_of_bounds(iter));
    }

    if iter.current().text != SOUL_NAMES.get_name(NamesOtherKeyWords::If) {
        return Err(new_soul_error(iter.current(), format!("Internal error: else if Statment doesn't start with '{}'", SOUL_NAMES.get_name(NamesOtherKeyWords::If)).as_str()));
    }

    if iter.next().is_none() {
        return Err(err_out_of_bounds(iter));
    }

    if iter.current().text == "{" {
        return Err(new_soul_error(iter.current(), "no condition in else if statment"));
    }

    const IS_FORWARD_DECLARED: bool = false;
    let expression_result = get_expression(iter, meta_data, context, &None, IS_FORWARD_DECLARED, &vec!["{"])?;
    
    if expression_result.is_type.name != SOUL_NAMES.get_name(NamesInternalType::Boolean) || 
       !expression_result.is_type.wrappers.is_empty() 
    {
        return Err(new_soul_error(iter.current(), format!("else if statment condition needs to be of type '{}' is type '{}'", SOUL_NAMES.get_name(NamesInternalType::Boolean), expression_result.is_type.to_string()).as_str()));
    }

    if iter.next().is_none() {
        return Err(new_soul_error(iter.current(), "unexpected end while trying to get if statment body"));
    }
    
    let body = get_body(iter, statment_iter, meta_data, context, None)?;
    if iter.next().is_none() {
        return Err(new_soul_error(iter.current(), "unexpected end while trying to get function body"));
    }
    
    let mut body_result = MultiStamentResult::new(IStatment::EmptyStatment());
    body_result.add_result(&expression_result.result);
    body_result.value = IStatment::new_if(expression_result.result.value, body, &iter[begin_i]);
    
    Ok(body_result)

}

fn get_else_statment(iter: &mut TokenIterator, statment_iter: &mut StatmentIterator, context: &mut CurrentContext, meta_data: &mut MetaData) -> Result<MultiStamentResult<IStatment>> {
    let begin_i = iter.current_index();
    if iter.current().text != SOUL_NAMES.get_name(NamesOtherKeyWords::Else) {
        return Err(new_soul_error(iter.current(), format!("Internal error: else Statment doesn't start with '{}'", SOUL_NAMES.get_name(NamesOtherKeyWords::Else)).as_str()));
    }
    if context.current_function.is_none() {
        return Err(new_soul_error(iter.current(), "trying to use else statment while not being in function"));
    }

    if iter.next().is_none() {
        return Err(err_out_of_bounds(iter));
    }

    if iter.current().text != "{" {
        return Err(new_soul_error(iter.current(), "else statment should start with '{'"));
    }

    if iter.next().is_none() {
        return Err(err_out_of_bounds(iter));
    }

    let body = get_body(iter, statment_iter, meta_data, context, None)?;
    if iter.next().is_none() {
        return Err(new_soul_error(iter.current(), "unexpected end while trying to get function body"));
    }
    
    Ok(MultiStamentResult::new(IStatment::new_else(body, &iter[begin_i])))
}

fn get_if_statment(iter: &mut TokenIterator, statment_iter: &mut StatmentIterator, context: &mut CurrentContext, meta_data: &mut MetaData) -> Result<MultiStamentResult<IStatment>> {
    let begin_i = iter.current_index();
    if iter.current().text != SOUL_NAMES.get_name(NamesOtherKeyWords::If) {
        return Err(new_soul_error(iter.current(), format!("Internal error: if Statment doesn't start with '{}'", SOUL_NAMES.get_name(NamesOtherKeyWords::If)).as_str()));
    }
    if context.current_function.is_none() {
        return Err(new_soul_error(iter.current(), "trying to use if statment while not being in function"));
    }

    if iter.next().is_none() {
        return Err(err_out_of_bounds(iter));
    }

    if iter.current().text == "{" {
        return Err(new_soul_error(iter.current(), "no condition in if statment"));
    }

    const IS_FORWARD_DECLARED: bool = false;
    let expression_result = get_expression(iter, meta_data, context, &None, IS_FORWARD_DECLARED, &vec!["{"])?;
    
    if expression_result.is_type.name != SOUL_NAMES.get_name(NamesInternalType::Boolean) || 
       !expression_result.is_type.wrappers.is_empty() 
    {
        return Err(new_soul_error(iter.current(), format!("if statment condition needs to be of type '{}' is type '{}'", SOUL_NAMES.get_name(NamesInternalType::Boolean), expression_result.is_type.to_string()).as_str()));
    }

    if iter.next().is_none() {
        return Err(new_soul_error(iter.current(), "unexpected end while trying to get if statment body"));
    }
    
    let body = get_body(iter, statment_iter, meta_data, context, None)?;
    if iter.next().is_none() {
        return Err(new_soul_error(iter.current(), "unexpected end while trying to get function body"));
    }
    
    let mut body_result = MultiStamentResult::new(IStatment::EmptyStatment());
    body_result.add_result(&expression_result.result);
    body_result.value = IStatment::new_if(expression_result.result.value, body, &iter[begin_i]);
    
    Ok(body_result)

}

fn get_return(iter: &mut TokenIterator, context: &mut CurrentContext, meta_data: &mut MetaData) -> Result<MultiStamentResult<IStatment>> {
    if iter.current().text != SOUL_NAMES.get_name(NamesOtherKeyWords::Return) {
        return Err(new_soul_error(iter.current(), format!("Internal error: return Statment doesn't start with '{}'", SOUL_NAMES.get_name(NamesOtherKeyWords::Return)).as_str()));
    }
    if context.current_function.is_none() {
        return Err(new_soul_error(iter.current(), "trying to return while not being in function"));
    }

    let return_type = context.current_function.as_ref().unwrap().return_type.clone();


    if iter.next().is_none() {
        return Err(err_out_of_bounds(iter));
    }

    if iter.current().text == "\n" || iter.current().text == ";" {

        if return_type.is_some() {
            return Err(new_soul_error(iter.current(), "trying to return without a returnType while function has a return type"));
        }

        return Ok(MultiStamentResult::new(
            IStatment::new_return(None, iter.current())
        ));
    }

    if return_type.is_none() {
        return Err(new_soul_error(iter.current(), "trying to return with a type while function does not have return type"));
    }

    let return_str = return_type.as_ref().unwrap();
    let return_type = SoulType::from_stringed_type(return_str, iter.current(), &meta_data.type_meta_data, &mut context.current_generics)?;    

    const IS_FORWARD_DECLARED: bool = false;
    let expression_result = get_expression(iter, meta_data, context, &Some(&return_type), IS_FORWARD_DECLARED, &vec!["\n", ";"])?;

    if !return_type.is_convertable(&expression_result.is_type, iter.current(), &meta_data.type_meta_data, &mut context.current_generics) {
        return Err(new_soul_error(iter.current(), format!("trying to return with a type: '{}' but can not be converted to function return type: '{}' ", expression_result.is_type.to_string(), return_str).as_str()));
    }

    let mut result = MultiStamentResult::new(IStatment::EmptyStatment());
    result.add_result(&expression_result.result);
    result.value = IStatment::new_return(Some(expression_result.result.value), iter.current());

    return Ok(result);
}

fn get_variable(iter: &mut TokenIterator, context: &mut CurrentContext, meta_data: &mut MetaData) -> Result<IVariable> {
    let var_name = iter.current();
    if let Err(msg) = check_name(&var_name.text) {
        return Err(new_soul_error(iter.current(), msg.as_str()));
    }

    let scope = meta_data.scope_store.get(&context.get_current_scope_id())
        .ok_or(new_soul_error(iter.current(), "Internal Error: could not get scope of: context.current_scope_id"))?;

    let variable = scope.try_get_variable(&var_name.text, &meta_data.scope_store)
        .ok_or(new_soul_error(iter.current(), format!("variable: '{}' could not be found in scope", var_name.text).as_str()))?;

    if iter.next().is_none() {
        return Err(err_out_of_bounds(iter));
    }

    Ok(IVariable::new_variable(&variable.0.name, &variable.0.type_name, iter.current()))
}

fn err_out_of_bounds(iter: &TokenIterator) -> SoulError {
    new_soul_error(iter.current(), "unexpected end while trying to get stament")
}










