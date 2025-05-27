use std::{collections::HashSet, io::{Error, Result}};
use once_cell::sync::Lazy;

use crate::{abstract_styntax_tree::{abstract_styntax_tree::{IStatment, IVariable}, get_abstract_syntax_tree::{get_body::get_body, get_expression::get_function_call::get_function_call::get_function_call, get_function_body::get_function_body, get_stament::{get_assignmet::get_assignment, get_initialize::get_initialize}, multi_stament_result::MultiStamentResult}}, meta_data::{convert_soul_error::convert_soul_error::new_soul_error, current_context::current_context::CurrentContext, meta_data::MetaData, soul_names::SOUL_NAMES, soul_type::soul_type::SoulType}, tokenizer::token::TokenIterator};
        
static ASSIGN_SYMBOOLS_SET: Lazy<HashSet<&&str>> = Lazy::new(|| {
    SOUL_NAMES.assign_symbools.iter().map(|(_, str)| str).collect::<HashSet<&&str>>()
});

pub fn get_statment(iter: &mut TokenIterator, meta_data: &mut MetaData, context: &mut CurrentContext, open_bracket_stack: &mut usize) -> Result<MultiStamentResult<IStatment>> {
    
    if iter.current().text == "\n" {

        if iter.next().is_none() {
            return Err(err_out_of_bounds(iter));
        }
    }
    
    if iter.current().text == "{" {
        let body = get_body(iter, meta_data, &context, None, open_bracket_stack)?;

        return Ok(MultiStamentResult::new(
            IStatment::new_scope(body)
        ));
    }
    else if iter.current().text == "}" {
        if *open_bracket_stack == 0 {
            return Err(new_soul_error(iter.current(), "one of your scopes in not closed (you have a '{' without a '}')"));
        }

        if iter.next().is_none() {
            return Err(err_out_of_bounds(iter));
        }
        
        *open_bracket_stack -= 1;
        return Ok(MultiStamentResult::new(IStatment::CloseScope()));
    }
    
    let mut is_wrong_type = false;
    let possible_type = SoulType::try_from_iterator(iter, &meta_data.type_meta_data, &context.current_generics, &mut is_wrong_type).ok();

    if possible_type.is_some() && !is_wrong_type  {

        let next_token = iter.peek()
            .ok_or(err_out_of_bounds(iter))?;

        let symbool;
        if next_token.text == ">" {
            symbool = get_symbool_after_generic(iter, iter.current_index())?;
        }
        else {
            symbool = &next_token.text;
        }

        if symbool == "=" {
            return get_initialize(iter, meta_data, context, open_bracket_stack);
        }
    }

    const FUNCTION_CALL: bool = false;
    let next = iter.peek()
        .ok_or(err_out_of_bounds(iter))?;
        
    match next.text.as_str() {
        ":=" => return get_initialize(iter, meta_data, context, open_bracket_stack),
        "(" => {
            if func_call_or_declaration(iter, meta_data, context)? == FUNCTION_CALL {
                return get_statment_function_call(iter, meta_data, context);
            }
            else {
                return get_function_body(iter, meta_data, context, open_bracket_stack);
            }          
        },
        _ => {
            if !ASSIGN_SYMBOOLS_SET.iter().any(|symb| symb == &&next.text) {
                return Err(new_soul_error(next, format!("token invalid for statment: '{}'", next.text).as_str()));
            }

            let var_info = meta_data.try_get_variable(&iter.current().text, &context.current_scope_id)
                .ok_or(new_soul_error(iter.current(), format!("trying to assign variable: '{}' but variable not found in scope", &iter.current().text).as_str()))?;

            let variable = IVariable::new_variable(&var_info.name, &var_info.type_name);

            let assign_result = get_assignment(iter, meta_data, context, variable)?;
            return Ok(assign_result.assignment)
        },
    }
}

fn get_statment_function_call(iter: &mut TokenIterator, meta_data: &mut MetaData, context: &mut CurrentContext) -> Result<MultiStamentResult<IStatment>> {
    let mut body_result = MultiStamentResult::new(IStatment::EmptyStatment());

    let function_call = get_function_call(iter, meta_data, context)?;
    body_result.add_result(&function_call);
    body_result.value = IStatment::new_function_call(function_call.value);

    iter.next();
    return Ok(body_result);
}

///true = func_declaration, false = func_call
fn func_call_or_declaration(iter: &mut TokenIterator, meta_data: &mut MetaData, context: &mut CurrentContext) -> Result<bool> {
    let begin_i = iter.current_index();
    
    go_to_symbool_after_brackets(iter, begin_i + 1);

    let is_curly_bracket = iter.current().text == "{";

    let mut is_wrong_type = false;
    let res = SoulType::try_from_iterator(iter, &meta_data.type_meta_data, &context.current_generics, &mut is_wrong_type);
    let is_type = res.is_ok() || is_wrong_type;

    iter.go_to_index(begin_i);
    Ok(is_type || is_curly_bracket)
}

fn go_to_symbool_after_brackets<'a>(iter: &mut TokenIterator, start_i: usize) {
    assert_eq!(&iter[start_i].text, "(");
    iter.go_to_index(start_i);

    loop {
        if iter.next().is_none() {
            break;
        }

        if iter.current().text == ")" {
            iter.next();
            break;
        }
    }
}

fn get_symbool_after_generic<'a>(iter: &'a TokenIterator, start_i: usize) -> Result<&'a str> {
    assert_eq!(iter[start_i].text , "<");
    let mut i = 0;

    loop {
        if i + start_i > iter.len() {
            return Err(new_soul_error(iter.current(), "unexpected end while trying to get generic (generic is not closed add '>')"));
        }

        let token = &iter[i + start_i];
        if token.text == ">" {
            i += 1;
            if i + start_i > iter.len() {
                return Err(new_soul_error(iter.current(), "unexpected end while trying to get generic (generic is not closed add '>')"));
            }

            break Ok(&iter[i + start_i].text);
        }

        i += 1;
    }

}

fn err_out_of_bounds(iter: &TokenIterator) -> Error {
    new_soul_error(iter.current(), "unexpected end while trying to get stament")
}


