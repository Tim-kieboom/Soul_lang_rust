use std::{collections::BTreeMap};
use crate::meta_data::soul_error::soul_error::{new_soul_error, Result, SoulError};

use crate::{abstract_styntax_tree::{abstract_styntax_tree::{BodyNode, IStatment}, get_abstract_syntax_tree::get_stament::get_statment::get_statment}, meta_data::{current_context::current_context::CurrentContext, function::{function_declaration::function_declaration::FunctionDeclaration}, meta_data::{CloseScopeResult, MetaData}, scope_and_var::var_info::{VarFlags, VarInfo}, soul_type::soul_type::SoulType}, tokenizer::token::TokenIterator};

use super::get_stament::statment_type::statment_type::StatmentIterator;

pub fn get_body(iter: &mut TokenIterator, statment_iter: &mut StatmentIterator, meta_data: &mut MetaData, old_context: &CurrentContext, possible_function: Option<FunctionDeclaration>) -> Result<BodyNode> {
    let begin_i = iter.current_index();

    let result = internal_get_body(iter, statment_iter, meta_data, old_context, possible_function);
    if result.is_err() {
        iter.go_to_index(begin_i);
    } 

    result
}

fn internal_get_body(iter: &mut TokenIterator, statment_iter: &mut StatmentIterator, meta_data: &mut MetaData, old_context: &CurrentContext, possible_function: Option<FunctionDeclaration>) -> Result<BodyNode> {
    if iter.next().is_none() {
        return Err(err_out_of_bounds(iter));
    }

    let scope_id = meta_data.open_scope(old_context.current_scope_id, possible_function.is_none())
        .map_err(|msg| new_soul_error(iter.current(), format!("while trying to add scope\n{}", msg).as_str()))?;

    let mut context = old_context.clone();
    context.current_scope_id = scope_id;

    let vars = if let Some(function) = &possible_function {
        function.args
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

                (name.clone(), VarInfo::with_var_flag(name.clone(), arg.value_type.clone(), var_flags, false))
            })
            .collect::<BTreeMap<String, VarInfo>>()
    }
    else {
        BTreeMap::new()
    };

    for (name, var) in &vars {
        meta_data.add_to_scope(var.clone(), &scope_id)
            .map_err(|msg| new_soul_error(iter.current(), format!("while adding argument: '{}' to scope\n{}", name, msg).as_str()))?;
    }

    context.current_function = possible_function;
    

    meta_data.scope_store.get_mut(&context.current_scope_id).unwrap().vars = vars;

    if iter.next_multiple(-1).is_none() {
        return Err(err_out_of_bounds(iter));
    }

    let mut has_return = false;
    let mut body_node = BodyNode::new(context);
    loop {
        let multi_statment = get_statment(iter, statment_iter, meta_data, &mut body_node.context)?;
        match &multi_statment.value {
            IStatment::CloseScope() => break,
            IStatment::Return{..} => has_return = true,
            _ => (),
        }

        body_node.statments.extend(multi_statment.before.into_iter().flatten()); 
        body_node.statments.push(multi_statment.value);
        body_node.statments.extend(multi_statment.after.into_iter().flatten());
    }

    let CloseScopeResult{delete_list, parent:_} = meta_data.close_scope(&scope_id)
        .map_err(|msg| new_soul_error(iter.current(), format!("while trying to clode scope\n{}", msg).as_str()))?;

    body_node.delete_list = delete_list;
    if let Some(function) = &body_node.context.current_function {
        
        if !has_return && function.return_type.is_some() {
            return Err(new_soul_error(iter.current(), format!("function: '{}' has return type but does not return anything", function.name).as_str()));
        }
    }    

    Ok(body_node)
}

fn err_out_of_bounds(iter: &TokenIterator) -> SoulError {
    new_soul_error(iter.current(), "unexpected end while trying to get body")
}








