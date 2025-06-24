use crate::meta_data::soul_error::soul_error::{new_soul_error, pass_soul_error, Result, SoulSpan};
use crate::{abstract_styntax_tree::{abstract_styntax_tree::{IStatment, IVariable}, get_abstract_syntax_tree::{get_expression::get_expression::get_expression, multi_stament_result::MultiStamentResult}}, meta_data::{current_context::current_context::CurrentContext, meta_data::MetaData, scope_and_var::var_info::{VarFlags, VarInfo}, soul_names::check_name, soul_type::{primitive_types::{NumberCategory, PrimitiveType}, soul_type::SoulType, type_modifiers::TypeModifiers}}, tokenizer::token::TokenIterator};

use super::get_assignmet::{get_assignment, AssignmentResult};

pub fn get_forward_declared_initialize(iter: &mut TokenIterator, meta_data: &mut MetaData, context: &mut CurrentContext, forward_declared_bracket_stack: i64) -> Result<MultiStamentResult<IStatment>> {
    let begin_i = iter.current_index();

    let result = internal_get_initialize(iter, meta_data, context, Some(forward_declared_bracket_stack));
    if result.is_err() {
        iter.go_to_index(begin_i);
    }

    result
}

pub fn get_initialize(iter: &mut TokenIterator, meta_data: &mut MetaData, context: &mut CurrentContext) -> Result<MultiStamentResult<IStatment>> {
    let begin_i = iter.current_index();

    let result = internal_get_initialize(iter, meta_data, context, None);
    if result.is_err() {
        iter.go_to_index(begin_i);
    }

    result
}

fn internal_get_initialize(iter: &mut TokenIterator, meta_data: &mut MetaData, context: &mut CurrentContext, forward_declared_bracket_stack: Option<i64>)  -> Result<MultiStamentResult<IStatment>> {

    let mut body_result = MultiStamentResult::new(IStatment::EmptyStatment(SoulSpan::from_token(iter.current())));

    let err_out_of_bounds = |iter: &TokenIterator| {
        new_soul_error(iter.current(), "unexpected error while getting initialize variable")
    };

    let possible_type = SoulType::from_iterator(iter, &meta_data.type_meta_data, &context.current_generics);
    let is_type_invered = possible_type.is_err();
    let mut modifier = TypeModifiers::Default;

    if is_type_invered {
        let modi = TypeModifiers::from_str(&iter.current().text);
        if modi != TypeModifiers::Default {
            modifier = modi;

            if iter.next().is_none() {
                return Err(err_out_of_bounds(iter));
            }
        }
    }
    else {
        modifier = possible_type.as_ref()
            .unwrap()
            .modifiers
            .clone();

        if iter.next().is_none() {
            return Err(err_out_of_bounds(iter));
        }
    }

    let var_name_index = iter.current_index();
    if let Err(msg) = check_name(&iter[var_name_index].text) {
        return Err(new_soul_error(iter.current(), msg.as_str()));
    }

    let possible_var = meta_data.scope_store.get(&context.get_current_scope_id())
        .unwrap()
        .try_get_variable_current_scope_only(&iter[var_name_index].text);
   
    let (already_exists, is_forward_declared) = match &possible_var {
        Some(var) => (true, var.is_forward_declared),
        None => (false, true),
    };

    if is_forward_declared {
        meta_data.scope_store.get_mut(&context.get_current_scope_id())
            .unwrap()
            .remove_variable_current_scope_only(&iter[var_name_index].text);
    }
    else {
        if already_exists {
            return Err(new_soul_error(iter.current(), format!("variable '{}' already exists in scope", &iter[var_name_index].text).as_str()));
        }
    }
    
    if !context.rulesets.is_mutable() && modifier.is_mutable() {
        return Err(new_soul_error(iter.current(), format!("variable '{}' is mutable but current ruleset ", &iter[var_name_index].text).as_str()));
    }

    if iter.next().is_none() {
        return Err(err_out_of_bounds(iter));
    }

    if iter.current().text == "\n" || iter.current().text == ";" {
        if is_type_invered {
            return Err(new_soul_error(iter.current(), format!("variable: '{}' is not assign a type (add type before variable like 'i32 var')", &iter[var_name_index].text).as_str()));
        }

        let var_type = possible_type.as_ref().unwrap();
        body_result.value = IStatment::new_initialize(
            IVariable::Variable {
                name: iter[var_name_index].text.clone(), 
                type_name: var_type.to_string(),
                span: SoulSpan::from_token(&iter[var_name_index])
            }, 
            None,
            iter.current()
        );
        
        let var_flags = get_var_flags(&var_type);
        let var_info = VarInfo::with_var_flag(
            iter[var_name_index].text.clone(), 
            var_type.to_string(),
            var_flags,
            forward_declared_bracket_stack.is_some()
        );

        add_to_scope(iter, meta_data, context, var_name_index, var_info)?;

        return Ok(body_result);
    }

    if is_type_invered {

        if modifier == TypeModifiers::Default && 
            iter.current().text != ":=" 
        {
            return Err(new_soul_error(iter.current(), format!("'{}' is not allowed at end of default type invered initialize variable (use ':=')", &iter.current().text).as_str()));
        }
    }
    else if iter.current().text != "=" {
        return Err(new_soul_error(iter.current(), format!("'{}' is not allowed at end of initialize variable (use '=')", &iter.current().text).as_str()));
    }

    if is_type_invered {
        if iter.next().is_none() {
            return Err(err_out_of_bounds(iter));
        }

        let begin_i = iter.current_index();
        let mut expression = get_expression(iter, meta_data, context, &None, forward_declared_bracket_stack.is_some(), &vec![";", "\n"])
            .map_err(|err| pass_soul_error(&iter[begin_i], format!("while trying to get assignment of variable: '{}'", &iter[var_name_index].text).as_str(), err))?;

        if expression.is_type.is_empty() {
            return Err(new_soul_error(iter.current(), format!("assignment type if variable: '{}' is of type 'none' which is not allowed", &iter[var_name_index].text).as_str()));
        }

        let mut primitive_expr_type = expression.is_type.to_primitive_type(&meta_data.type_meta_data); 
        if primitive_expr_type.is_untyped_type() {
            
            primitive_expr_type = match primitive_expr_type.to_number_category() {
                NumberCategory::Invalid => panic!("Internal error: untyped_type is not a number"),
                NumberCategory::Interger => PrimitiveType::Int,
                NumberCategory::UnsignedInterger => PrimitiveType::Uint,
                NumberCategory::FloatingPoint => PrimitiveType::F32,
            };

            expression.is_type.name = primitive_expr_type.to_str()
                .expect("Internal error: Primitive type was not able to be converted into str")
                .to_string();

        }

        if !modifier.contains(TypeModifiers::Literal) {
            expression.is_type.remove_modifier(TypeModifiers::Literal);
        }
        else if !modifier.contains(TypeModifiers::Const) {
            expression.is_type.remove_modifier(TypeModifiers::Const);
        }

        body_result.add_result(&expression.result);
        expression.is_type.add_modifier(modifier)
            .map_err(|msg| new_soul_error(iter.current(), &msg))?;

        let mut var_flags = get_var_flags(&expression.is_type);
        var_flags |= VarFlags::IsAssigned;


        let var_info = VarInfo::with_var_flag(
            iter[var_name_index].text.to_string(), 
            expression.is_type.to_string(), 
            var_flags,
            forward_declared_bracket_stack.is_some()
        );

        add_to_scope(iter, meta_data, context, var_name_index, var_info)?;

        let variable = IVariable::Variable { 
            name: iter[var_name_index].text.clone(), 
            type_name: expression.is_type.to_string(),
            span: SoulSpan::from_token(&iter[var_name_index])
        };

        body_result.value = IStatment::new_initialize(
            variable.clone(), 
            Some(IStatment::new_assignment(variable, expression.result.value, iter.current())?),
            iter.current()
        );

        Ok(body_result)
    }
    else {
        let var_type = possible_type.unwrap();
        let variable = IVariable::Variable {
            name: iter[var_name_index].text.clone(), 
            type_name: var_type.to_string(),
            span: SoulSpan::from_token(&iter[var_name_index])
        };

        let mut var_flags = get_var_flags(&var_type);
        var_flags |= VarFlags::IsAssigned;

        let var_info = VarInfo::with_var_flag(
            iter[var_name_index].text.to_string(), 
            var_type.to_string(), 
            var_flags,
            forward_declared_bracket_stack.is_some()
        );

        add_to_scope(iter, meta_data, context, var_name_index, var_info)?;

        let AssignmentResult{assignment, is_type: _} = get_assignment(iter, meta_data, context, variable.clone(), true)?;
        body_result.add_result(&assignment);

        body_result.value = IStatment::new_initialize(variable, Some(assignment.value), iter.current());
        Ok(body_result)
    }
}

fn add_to_scope(iter: &mut TokenIterator, meta_data: &mut MetaData, context: &mut CurrentContext, var_name_index: usize, var_info: VarInfo) -> Result<()> {
    meta_data.add_to_scope(var_info, &context.get_current_scope_id())
        .map_err(|msg| new_soul_error(iter.current(), format!("while trying to add variable: '{}' to scope\n{}", iter[var_name_index].text, msg).as_str()))
}

fn get_var_flags(var_type: &SoulType) -> VarFlags {
    let mut var_flags = VarFlags::Empty;
    if var_type.is_mutable() {
        var_flags |= VarFlags::IsMutable;
    }
    if var_type.is_literal() {
        var_flags |= VarFlags::IsLiteral;
    }

    var_flags
}





