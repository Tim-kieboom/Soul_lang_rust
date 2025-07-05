use once_cell::sync::Lazy;
use super::get_stament::statment_type::statment_type::{StatmentIterator, StatmentType};
use crate::{abstract_styntax_tree::get_abstract_syntax_tree::get_stament::statment_type::statment_type::StatmentTypeInfo, meta_data::{soul_error::soul_error::{new_soul_error, pass_soul_error, Result}, soul_names::NamesOtherKeyWords}};
use crate::{abstract_styntax_tree::{abstract_styntax_tree::AbstractSyntaxTree, get_abstract_syntax_tree::get_stament::{get_statment::get_statment, statment_type::get_statment_types::get_statment_types}}, meta_data::{current_context::current_context::CurrentContext, meta_data::MetaData, soul_names::{NamesTypeModifiers, SOUL_NAMES}}, tokenizer::token::TokenIterator};

const GLOBAL_SCOPE: i64 = 0;
static ELSE: Lazy<String> = Lazy::new(|| SOUL_NAMES.get_name(NamesOtherKeyWords::Else).to_string());
static ELSE_IF: Lazy<String> = Lazy::new(|| format!("{} {}", SOUL_NAMES.get_name(NamesOtherKeyWords::Else), SOUL_NAMES.get_name(NamesOtherKeyWords::If)));

pub fn get_abstract_syntax_tree_file(mut iter: TokenIterator, meta_data: &mut MetaData) -> Result<(AbstractSyntaxTree, StatmentIterator)> {
    let mut context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);
    
    #[cfg(feature="dev_mode")]
    println!(
        "\ntokenizer:\n{:?}\n", 
        iter.get_tokens_text()
            .iter()
            .enumerate()
            .collect::<Vec<(usize, &&str)>>()
    );

    let statment_count = iter
        .ref_tokens().iter()
        .filter(|token| token.text == "\n" || token.text == ";")
        .count();

    let mut statment_type_info = StatmentTypeInfo::with_capacity(GLOBAL_SCOPE, statment_count);

    loop {
        let is_done = forward_declare(&mut iter, meta_data, &mut context, &mut statment_type_info)
            .map_err(|err| pass_soul_error(iter.current(), "while forward declaring", err))?;

        if is_done {
            break;
        }
    }

    iter.go_to_before_start();


    #[cfg(feature="dev_mode")]
    {
        use itertools::Itertools;
        use crate::meta_data::function::internal_functions::INTERNAL_FUNCTIONS;

        println!(
            "statment_types:\n{:#?}\n", 
            statment_type_info.statment_types
            .iter()
            .enumerate()
            .map(|(i, el)| format!("{}.{:?}", i, el))
            .collect::<Vec<String>>()
        );

        println!(
            "metaData.scopes (before parser):\n{:#?}\n",
            meta_data.scope_store
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.0.0, &b.0.0))
            .map(|(id, scope)| 
                format!(
                    "parent: {:?}, id: {}, funcs: {:?}, scopes: {:?}", 
                    scope.parent,
                    id.0,
                    scope.function_store.iter_names()
                    .filter(|name| !INTERNAL_FUNCTIONS.iter().any(|internal| &&internal.name == name))
                    .collect::<Vec<_>>(),
                    scope.vars.iter().map(|var| var.0).collect::<Vec<_>>(),
                ),
            )
            .collect::<Vec<_>>()
        );
    }
    
    context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);

    let mut statment_iter = StatmentIterator::new(statment_type_info.statment_types);
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
    
    Ok((tree, statment_iter))
}

pub fn get_abstract_syntax_tree_line(tree: &mut AbstractSyntaxTree, iter: &mut TokenIterator, context: &mut CurrentContext, meta_data: &mut MetaData, statment_info: &mut StatmentTypeInfo) -> Result<()> {
    let begin_i = iter.current_index();
    
    loop {
        let is_done = forward_declare(iter, meta_data, context, statment_info)
            .map_err(|err| pass_soul_error(iter.current(), "while forward declaring", err))?;

        if is_done {
            break;
        }
    }

    iter.go_to_index(begin_i);
    let mut statment_iter = StatmentIterator::new(std::mem::take(&mut statment_info.statment_types));
    loop {  
        let multi_statment = get_statment(iter, &mut statment_iter, meta_data, context)?;

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

    statment_info.statment_types = statment_iter.get_consume_statments();
    
    Ok(())
}

fn forward_declare(iter: &mut TokenIterator, meta_data: &mut MetaData, context: &mut CurrentContext, statment_info: &mut StatmentTypeInfo) -> Result<bool> {

    fn is_global_scope(statment_info: &StatmentTypeInfo) -> bool {
        statment_info.open_bracket_stack == GLOBAL_SCOPE
    }

    if iter.current().text == "\n" {

        if iter.next().is_none() {
            return Ok(true);
        }
    }

    let statment_type = get_statment_types(iter, meta_data, context, statment_info)?;
    match &statment_type {
        StatmentType::CloseScope{..} => (),
        StatmentType::EmptyStatment => (),
        StatmentType::Assignment => {
                if is_global_scope(statment_info) {
                    return Err(new_soul_error(iter.current(), "can not do an assignment in global scope"));
                }
            }
        StatmentType::TypeDef{..} => (),
        StatmentType::Initialize{is_mutable, is_assigned, var} => {
                if is_global_scope(statment_info) {
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
        StatmentType::Scope{..} => (),
        StatmentType::Return{..} => {
                if is_global_scope(statment_info) {
                    return Err(new_soul_error(iter.current(), "can not return in global scope"));
                }
            },
        StatmentType::If{..} => {
                if is_global_scope(statment_info) {
                    return Err(new_soul_error(iter.current(), "can not have an if statment in global scope"));
                }
            },
        StatmentType::Else{..} => check_else_statment(iter, statment_info, &ELSE)?,
        StatmentType::ElseIf{..} => check_else_statment(iter, statment_info, &ELSE_IF)?,
        StatmentType::While{..} => (),
        StatmentType::For{..} => (),
    }

    statment_info.statment_types.push(statment_type);

    if iter.next().is_none() {
        Ok(true)
    }
    else {
        Ok(false)
    }
}

fn check_else_statment(iter: &mut TokenIterator, statment_info: &StatmentTypeInfo, name: &str) -> Result<()> {
    let statments = &statment_info.statment_types;
    
    if let StatmentType::CloseScope{begin_body_index} = statments[statments.len() - 1] {
        match statments[begin_body_index] {
            StatmentType::If {..} => (),
            StatmentType::ElseIf {..} => (),

            _ => return Err(new_soul_error(iter.current(), format!("can not have an {} statment without an if or else if statment first", name).as_str())),
        }
    }
    else {
        return Err(new_soul_error(iter.current(), format!("can not have an {} statment without an if or else if statment first", name).as_str()));
    }

    Ok(())
} 













