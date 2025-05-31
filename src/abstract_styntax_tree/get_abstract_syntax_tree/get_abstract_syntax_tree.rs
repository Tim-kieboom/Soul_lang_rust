use std::io::Result;
use super::get_stament::statment_type::statment_type::{StatmentIterator, StatmentType};
use crate::{abstract_styntax_tree::{abstract_styntax_tree::AbstractSyntaxTree, get_abstract_syntax_tree::get_stament::{get_statment::get_statment, statment_type::get_statment_types::get_statment_types}}, meta_data::{convert_soul_error::convert_soul_error::new_soul_error, current_context::current_context::CurrentContext, meta_data::MetaData, soul_names::{NamesTypeModifiers, SOUL_NAMES}}, tokenizer::token::TokenIterator};

const GLOBAL_SCOPE: i64 = 0;

pub fn get_abstract_syntax_tree_file(mut iter: TokenIterator, meta_data: &mut MetaData) -> Result<AbstractSyntaxTree> {
    let mut context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);
    
    let mut statments = Vec::new();
    let mut open_bracket_stack = GLOBAL_SCOPE;
    loop {
        let is_done = forward_declare(&mut iter, meta_data, &mut context, &mut statments, &mut open_bracket_stack)?;
        if is_done {
            break;
        }
    }
    
    iter.go_to_before_start();

    println!("{:#?}", statments);
    println!("{:?}", iter.get_tokens_text().iter().enumerate().collect::<Vec<_>>());
    
    let mut statment_iter = StatmentIterator::new(statments);
    let mut tree = AbstractSyntaxTree::new();
    loop {

        let multi_statment = get_statment(&mut iter, &mut statment_iter, meta_data, &mut context)?;
        
        tree.main_nodes.extend(multi_statment.before.into_iter().flatten());
        tree.main_nodes.push(multi_statment.value);
        tree.main_nodes.extend(multi_statment.after.into_iter().flatten());

        if iter.peek().is_none() {
            break;
        }
    }
    
    Ok(tree)
}

pub fn get_abstract_syntax_tree_line(tree: &mut AbstractSyntaxTree, iter: &mut TokenIterator, statment_iter: &mut StatmentIterator, context: &mut CurrentContext, meta_data: &mut MetaData, open_bracket_stack: &mut i64) -> Result<()> {
    let begin_i = iter.current_index();
    
    loop {
        let is_done = forward_declare(iter, meta_data, context, statment_iter.get_statments_mut(), open_bracket_stack)?;
        if is_done {
            break;
        }
    }

    iter.go_to_index(begin_i);

    loop {

        let multi_statment = get_statment(iter, statment_iter, meta_data, context)?;
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

fn forward_declare(iter: &mut TokenIterator, meta_data: &mut MetaData, context: &mut CurrentContext, statments: &mut Vec<StatmentType>, open_bracket_stack: &mut i64) -> Result<bool> {

    if iter.current().text == "\n" {

        if iter.next().is_none() {
            return Ok(true);
        }
    }

    let statment_type = get_statment_types(iter, meta_data, context, open_bracket_stack)?;
    match &statment_type {
        StatmentType::CloseScope => (),
        StatmentType::EmptyStatment => (),
        StatmentType::Assignment => {
            if *open_bracket_stack == GLOBAL_SCOPE {
                return Err(new_soul_error(iter.current(), "can not do an assignment in global scope"));
            }
        }
        StatmentType::Initialize{is_mutable, is_assigned, var} => {
            if *open_bracket_stack == GLOBAL_SCOPE {
                if !*is_assigned {
                    return Err(new_soul_error(iter.current(), format!("global variable: '{}' HAS TO BE assigned", var.get_name()).as_str()));
                }

                if *is_mutable {
                    return Err(new_soul_error(iter.current(), format!("global variable: '{}' can not be mutable has to be '{}' or '{}'", var.get_name(), SOUL_NAMES.get_name(NamesTypeModifiers::Constent), SOUL_NAMES.get_name(NamesTypeModifiers::Literal)).as_str()));
                }
            }
        },
        StatmentType::FunctionBody{..} => (),
        StatmentType::FunctionCall => (),
        StatmentType::Scope => (),
    }

    statments.push(statment_type);

    if iter.next().is_none() {
        Ok(true)
    }
    else {
        Ok(false)
    }
}














