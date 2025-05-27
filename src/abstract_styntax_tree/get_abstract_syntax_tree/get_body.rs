use std::{collections::BTreeMap, io::Result};
use crate::{abstract_styntax_tree::{abstract_styntax_tree::{BodyNode, IStatment}, get_abstract_syntax_tree::get_stament::get_statment::get_statment}, meta_data::{convert_soul_error::convert_soul_error::new_soul_error, current_context::current_context::CurrentContext, meta_data::MetaData, scope_and_var::var_info::VarInfo}, tokenizer::token::TokenIterator};

pub fn get_body(iter: &mut TokenIterator, meta_data: &mut MetaData, old_context: &CurrentContext, possible_arguments: Option<BTreeMap<String, VarInfo>>, open_bracket_stack: &mut usize) -> Result<BodyNode> {
    let begin_i = iter.current_index();

    let result = internal_get_body(iter, meta_data, old_context, possible_arguments, open_bracket_stack);
    if result.is_err() {
        iter.go_to_index(begin_i);
    } 

    result
}

fn internal_get_body(iter: &mut TokenIterator, meta_data: &mut MetaData, old_context: &CurrentContext, possible_arguments: Option<BTreeMap<String, VarInfo>>, open_bracket_stack: &mut usize) -> Result<BodyNode> {

    let scope_id = meta_data.open_scope(old_context.current_scope_id)
        .map_err(|msg| new_soul_error(iter.current(), format!("while trying to add scope\n{}", msg).as_str()))?;

    let mut context = old_context.clone();
    context.current_scope_id = scope_id;

    let vars = possible_arguments
        .unwrap_or(BTreeMap::new());

    meta_data.scope_store.get_mut(&context.current_scope_id).unwrap().vars = vars;
    
    let mut body_node = BodyNode::new(context);
    *open_bracket_stack += 1;
    loop {
        let multi_statment = get_statment(iter, meta_data, &mut body_node.context, open_bracket_stack)?;
        if let IStatment::CloseScope() = multi_statment.value {
            break;
        }

        body_node.statments.extend(multi_statment.before.into_iter().flatten()); 
        body_node.statments.push(multi_statment.value);
        body_node.statments.extend(multi_statment.after.into_iter().flatten());
    }

    body_node.delete_list = meta_data.close_scope(&scope_id)
        .map_err(|msg| new_soul_error(iter.current(), format!("while trying to clode scope\n{}", msg).as_str()))?;

    Ok(body_node)
}



