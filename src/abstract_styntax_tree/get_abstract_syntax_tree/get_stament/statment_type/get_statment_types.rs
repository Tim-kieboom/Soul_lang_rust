use std::{collections::HashSet, io::{Error, Result}};
use once_cell::sync::Lazy;

use crate::{abstract_styntax_tree::{abstract_styntax_tree::IStatment, get_abstract_syntax_tree::get_stament::get_initialize::get_initialize}, meta_data::{convert_soul_error::convert_soul_error::new_soul_error, current_context::current_context::CurrentContext, function::function_declaration::get_function_declaration::add_function_declaration, meta_data::MetaData, soul_names::SOUL_NAMES, soul_type::{soul_type::SoulType, type_modifiers::TypeModifiers}}, tokenizer::token::TokenIterator};

use super::statment_type::StatmentType;

static ASSIGN_SYMBOOLS_SET: Lazy<HashSet<&&str>> = Lazy::new(|| {
    SOUL_NAMES.assign_symbools.iter().map(|(_, str)| str).collect::<HashSet<&&str>>()
});

///get statment type of soul before helper symbools are added
pub fn get_statment_types(iter: &mut TokenIterator, meta_data: &mut MetaData, context: &mut CurrentContext, open_bracket_stack: &mut i64) -> Result<StatmentType> {
    if iter.current().text == "\n" {
        
        if iter.next().is_none() {
            return Err(err_out_of_bounds(iter));
        }
    }
    
    if iter.current().text == "{" {
        *open_bracket_stack += 1;
        if iter.next().is_none() {
            return Err(err_out_of_bounds(iter));
        }
        return Ok(StatmentType::Scope);
    }
    else if iter.current().text == "}" {
        if *open_bracket_stack < 0 {
            return Err(new_soul_error(iter.current(), "one of your scopes in not closed (you have a '{' without a '}')"));
        }

        if iter.next().is_none() {
            return Err(err_out_of_bounds(iter));
        }
        
        *open_bracket_stack -= 1;
        return Ok(StatmentType::CloseScope);
    }
    
    let mut is_wrong_type = false;
    let possible_type = SoulType::try_from_iterator(iter, &meta_data.type_meta_data, &context.current_generics, &mut is_wrong_type).ok();

    if possible_type.is_some() && !is_wrong_type  {

        let next_token = iter.peek()
            .ok_or(err_out_of_bounds(iter))?;

        let symbool;
        if next_token.text == ">" {
            let begin_i = iter.current_index();
            get_symbool_after_generic(iter, iter.current_index())?;
            iter.go_to_index(begin_i);

            symbool = &iter.current().text;
        }
        else {
            symbool = &next_token.text;
        }

        if symbool == "=" {
            return Ok(get_initialize_info(iter, meta_data, context)?);
        }
    }

    let peek_i: i64;
    let type_i = iter.current_index();
    if let Ok(_) = SoulType::from_iterator(iter, &meta_data.type_meta_data, &context.current_generics) {
        peek_i = iter.current_index() as i64 - type_i as i64;
    }
    else {
        if TypeModifiers::from_str(&iter.current().text) != TypeModifiers::Default {
            peek_i = 2;
        }
        else {
            peek_i = 1;
        }
    }
    iter.go_to_index(type_i);

    const FUNCTION_CALL: bool = false;
    let next = iter.peek_multiple(peek_i)
        .ok_or(err_out_of_bounds(iter))?;
        
    match next.text.as_str() {
        "=" => {
            if peek_i != 1 {
                return Ok(get_initialize_info(iter, meta_data, context)?);
            }
        }
        ":=" => return Ok(get_initialize_info(iter, meta_data, context)?),
        "(" => {
            let begin_i = iter.current_index();
            if func_call_or_declaration(iter, meta_data, context)? == FUNCTION_CALL {
                return Ok(StatmentType::FunctionCall);
            }
            else {
                iter.go_to_index(begin_i);
                let func_info = add_function_declaration(iter, meta_data, context)?;
                *open_bracket_stack += 1;
                if iter.next().is_none() {
                    return Err(err_out_of_bounds(iter));
                }
                
                return Ok(StatmentType::FunctionBody{func_info});
            }          
        },
        _ => (),
    }

    if !ASSIGN_SYMBOOLS_SET.iter().any(|symb| symb == &&next.text) {
        return Err(new_soul_error(next, format!("token invalid for statment: '{}'", next.text).as_str()));
    }

    traverse_assignment(iter)?;
    return Ok(StatmentType::Assignment)
}

fn traverse_assignment(iter: &mut TokenIterator) -> Result<()> {
    loop {
        if iter.next().is_none() {
            break Err(new_soul_error(iter.current(), "unexpected end while trying to get assignment (add enter or ';')"));
        }

        let str = &iter.current().text;
        if str == "\n" || str == ";" {
            break Ok(());
        }
    }
}

///true = func_declaration, false = func_call
fn func_call_or_declaration(iter: &mut TokenIterator, meta_data: &mut MetaData, context: &mut CurrentContext) -> Result<bool> {
    go_to_symbool_after_brackets(iter, iter.current_index() + 1)?;

    let is_curly_bracket = iter.current().text == "{";

    let mut is_wrong_type = false;
    let res = SoulType::try_from_iterator(iter, &meta_data.type_meta_data, &context.current_generics, &mut is_wrong_type);
    let is_type = res.is_ok() || is_wrong_type;

    Ok(is_type || is_curly_bracket)
}

fn go_to_symbool_after_brackets<'a>(iter: &mut TokenIterator, start_i: usize) -> Result<()> {
    if &iter[start_i].text != "(" {
        return Err(new_soul_error(iter.current(), "unexpected start while trying to get args (args is not opened add '(')"));
    }

    iter.go_to_index(start_i);
    let mut stack = 1;

    loop {
        if iter.next().is_none() {
            break Err(new_soul_error(iter.current(), "unexpected end while trying to get args (args is not closed add ')')"));
        }

        if iter.current().text == "(" {
            stack += 1;
        }
        else if iter.current().text == ")" {
            stack -= 1;
        }

        if stack == 0 {
            iter.next();
            break Ok(());
        }
    }
}

fn get_symbool_after_generic<'a>(iter: &'a mut TokenIterator, start_i: usize) -> Result<()> {
    if &iter[start_i].text != "<" {
        return Err(new_soul_error(iter.current(), "unexpected start while trying to get generic (generic is not opened add '<')"));
    }

    iter.go_to_index(start_i);
    let mut stack = 1;

    loop {
        if iter.next().is_none() {
            break Err(new_soul_error(iter.current(), "unexpected end while trying to get generic (generic is not closed add '>')"));
        }

        if iter.current().text == "<" {
            stack += 1;
        }
        else if iter.current().text == ">" {
            stack -= 1;
        }

        if stack == 0 {
            iter.next();
            break Ok(());
        }
    }
}

fn get_initialize_info(iter: &mut TokenIterator, meta_data: &mut MetaData, context: &mut CurrentContext) -> Result<StatmentType> {
    let init = get_initialize(iter, meta_data, context)?;
    let var;
    let is_mutable;
    let is_assigned;
    if let IStatment::Initialize { variable, assignment } = init.value {
        let var_type = SoulType::get_unchecked_from_stringed_type(&variable.get_type_name(), iter.current(), &meta_data.type_meta_data, &mut context.current_generics)?;
        is_mutable = var_type.is_mutable();
        is_assigned = assignment.is_some();
        var = variable;
    }
    else {
        return Err(new_soul_error(iter.current(), "Internal error: get_initialize did not return IStatment::Initialize"));
    }

    Ok(StatmentType::Initialize{is_assigned, is_mutable, var})
}

fn err_out_of_bounds(iter: &TokenIterator) -> Error {
    new_soul_error(iter.current(), "unexpected end while trying to get stament")
}

















