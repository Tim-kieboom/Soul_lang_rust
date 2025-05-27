use std::io::Result;
use crate::{abstract_styntax_tree::{abstract_styntax_tree::{AbstractSyntaxTree, IStatment}, get_abstract_syntax_tree::get_stament::get_statment::get_statment}, meta_data::{convert_soul_error::convert_soul_error::new_soul_error, current_context::current_context::CurrentContext, function::function_declaration::get_function_declaration::add_function_declaration, meta_data::MetaData, scope_and_var::var_info::{VarFlags, VarInfo}, soul_names::{NamesTypeModifiers, SOUL_NAMES}, soul_type::soul_type::SoulType}, tokenizer::token::TokenIterator};

use super::get_stament::get_initialize::get_initialize;

#[allow(dead_code)]
pub fn get_abstract_syntax_tree_file(iter: &mut TokenIterator, meta_data: &mut MetaData) -> Result<AbstractSyntaxTree> {
    let mut context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);
    
    let begin_i = iter.current_index();
    loop {
        let is_done = forward_declare(iter, meta_data, &mut context)?;
        if is_done {
            break;
        }
    }
    iter.go_to_index(begin_i);

    let mut tree = AbstractSyntaxTree::new();
    let mut open_bracket_stack = 0;
    loop {

        let multi_statment = get_statment(iter, meta_data, &mut context, &mut open_bracket_stack)?;
        tree.main_nodes.extend(multi_statment.before.into_iter().flatten());
        tree.main_nodes.push(multi_statment.value);
        tree.main_nodes.extend(multi_statment.after.into_iter().flatten());

        if iter.current().text == "\n" {

            if iter.next().is_none() {
                break;
            }
        }

        if iter.next().is_none() {
            break;
        }
    }
    
    Ok(tree)
}

#[allow(dead_code)]
pub fn get_abstract_syntax_tree_line(tree: &mut AbstractSyntaxTree, iter: &mut TokenIterator, context: &mut CurrentContext, meta_data: &mut MetaData, open_bracket_stack: &mut usize) -> Result<()> {
    
    loop {

        let multi_statment = get_statment(iter, meta_data, context, open_bracket_stack)?;
        tree.main_nodes.extend(multi_statment.before.into_iter().flatten());
        tree.main_nodes.push(multi_statment.value);
        tree.main_nodes.extend(multi_statment.after.into_iter().flatten());

        if iter.current().text == "\n" {

            if iter.next().is_none() {
                break;
            }
        }

        if iter.next().is_none() {
            break;
        }
    }
    
    Ok(())
}

fn forward_declare(iter: &mut TokenIterator, meta_data: &mut MetaData, context: &mut CurrentContext) -> Result<bool> {

    if iter.current().text == "\n" {

        if iter.next().is_none() {
            return Ok(true);
        }
    }

    let result = add_function_declaration(iter, meta_data, context);
    if result.is_err() {
        let initialize = get_initialize(iter, meta_data, context)?;
        if let IStatment::Initialize { variable, assignment } = initialize.value {
            if assignment.is_none() {
                return Err(new_soul_error(iter.current(), format!("global variable: '{}' HAS TO BE assigned", variable.get_name()).as_str()));
            }

            let (name, type_name) = (variable.get_name(), variable.get_type_name());
            let soul_type = SoulType::get_unchecked_from_stringed_type(&type_name, iter.current(), &meta_data.type_meta_data, &mut context.current_generics)?;

            let mut var_flag = VarFlags::Empty;
            if soul_type.is_mutable() {
                return Err(new_soul_error(iter.current(), format!("global variable: '{}' can not be mutable has to be '{}' or '{}'", variable.get_name(), SOUL_NAMES.get_name(NamesTypeModifiers::Constent), SOUL_NAMES.get_name(NamesTypeModifiers::Literal)).as_str()));
            }

            var_flag |= VarFlags::IsAssigned;
            if soul_type.is_literal() {
                var_flag |= VarFlags::IsLiteral;
            }

            meta_data.add_to_scope(VarInfo::with_var_flag(name.to_string(), type_name.to_string(), var_flag), &MetaData::GLOBAL_SCOPE_ID);
        }
    }
    else {
        if iter.next().is_none() {
            return Ok(true);
        }

        if iter.current().text != "{" {
            SoulType::from_iterator(iter, &meta_data.type_meta_data, &context.current_generics)?;
        }

        traverse_through_scope(iter);
    }


    if iter.next().is_none() {
        return Ok(true);
    }

    Ok(false)
}

fn traverse_through_scope(iter: &mut TokenIterator) {
    let mut open_btacket_stack = 1;

    loop {
        if iter.next().is_none() {
            break;
        }

        if iter.current().text == "{" {
            open_btacket_stack += 1;
        }

        if iter.current().text == "}" {
            open_btacket_stack -= 1;
        }

        if open_btacket_stack == 0 {
            break;
        }
    }
}














