use std::collections::BTreeMap;

use crate::tokenizer::token::Token;
use crate::cpp_transpiller::cpp_type::CppType;
use crate::meta_data::scope_and_var::scope::ScopeId;
use crate::meta_data::soul_type::soul_type::SoulType;
use crate::cpp_transpiller::namespace::get_scope_namespace;
use crate::meta_data::soul_type::type_wrappers::TypeWrappers;
use crate::meta_data::soul_type::type_modifiers::TypeModifiers;
use crate::abstract_styntax_tree::operator_type::ExprOperatorType;
use crate::abstract_styntax_tree::abstract_styntax_tree::IVariable;
use crate::cpp_transpiller::convert_to_cpp::variable_to_cpp::variable_to_cpp;
use crate::meta_data::soul_error::soul_error::{new_soul_error, Result, SoulSpan};
use crate::{abstract_styntax_tree::abstract_styntax_tree::IExpression, cpp_transpiller::cpp_writer::CppWriter, meta_data::{current_context::current_context::CurrentContext, meta_data::MetaData}};

pub fn expression_to_cpp(writer: &mut CppWriter, expression: &IExpression, meta_data: &MetaData, context: &CurrentContext, in_scope_id: ScopeId) -> Result<()> {

    match expression {
        IExpression::IVariable{this, ..} => writer.push_str(&this.get_name()),
        IExpression::BinairyExpression{..} => binary_expression_to_cpp(writer, expression, meta_data, context, in_scope_id)?,
        IExpression::Literal{..} => literal_to_cpp(writer, expression, meta_data, context)?,
        IExpression::ConstRef{..} => ref_to_cpp(writer, expression, meta_data, context, in_scope_id)?,
        IExpression::MutRef{..} => ref_to_cpp(writer, expression, meta_data, context, in_scope_id)?,
        IExpression::DeRef{..} => deref_to_cpp(writer, expression, meta_data, context, in_scope_id)?,
        IExpression::Increment{..} => inrement_to_cpp(writer, expression, meta_data, context)?,
        IExpression::FunctionCall{..} => function_call_to_cpp(writer, expression, meta_data, context, in_scope_id)?,
        IExpression::EmptyExpression(_) => (),
    }

    Ok(())
}

fn function_call_to_cpp(writer: &mut CppWriter, expression: &IExpression, meta_data: &MetaData, context: &CurrentContext, in_scope_id: ScopeId) -> Result<()> {
    let (args, function_info, generic_defines, span) = match expression {
        IExpression::FunctionCall{ args, function_info, generic_defines, span } => (args, function_info, generic_defines, span),
        _ => return Err(new_soul_error(&token_from_span(expression.get_span()), "Internal error deref_to_cpp() called while statment is not DeRef")),
    };

    if function_info.in_scope_id != MetaData::GLOBAL_SCOPE_ID {
        get_scope_namespace(writer, &function_info.in_scope_id);
        writer.push_str("::");
    }

    writer.push_str(&function_info.name);
    generic_define_to_cpp(writer, generic_defines, span, meta_data, context)?;
    writer.push('(');
    for (i, arg) in args.iter().enumerate() {
        expression_to_cpp(writer, arg, meta_data, context, in_scope_id)?;

        if i < args.len()-1 {
            writer.push(',');
        }
    }
    writer.push(')');
    

    Ok(())
}

fn generic_define_to_cpp(writer: &mut CppWriter, generic_defines: &BTreeMap<String, String>, span: &SoulSpan, meta_data: &MetaData, context: &CurrentContext) -> Result<()> {
    if !generic_defines.is_empty() {
        return Ok(());   
    }

    for (i, (_generic_name, type_name)) in generic_defines.iter().enumerate() {
        let soul_type = SoulType::from_stringed_type(type_name, &token_from_span(span), &meta_data.type_meta_data, &context.current_generics)?;
        writer.push_str(CppType::from_soul_type(&soul_type, meta_data, context, span)?.as_str());
        
        if i > generic_defines.len() -1 {
            writer.push(',');
        }
    }

    Ok(())
}

fn inrement_to_cpp(writer: &mut CppWriter, expression: &IExpression, meta_data: &MetaData, context: &CurrentContext) -> Result<()> {
    let (amount, is_before, variable, span) = match expression {
        IExpression::Increment{ amount, is_before, variable, span } => (amount, is_before, variable, span),
        _ => return Err(new_soul_error(&token_from_span(expression.get_span()), "Internal error inrement_to_cpp() called while statment is not Increment")),
    };

    let soul_type = SoulType::from_stringed_type(variable.get_type_name(), &token_from_span(span), &meta_data.type_meta_data, &context.current_generics)?;

    if *is_before {
        if amount < &0 {
            operator_to_cpp(writer, &ExprOperatorType::Decrement, &soul_type, span)?;
        }
        else {            
            operator_to_cpp(writer, &ExprOperatorType::Increment, &soul_type, span)?;
        }
        variable_to_cpp(writer, variable, meta_data, context)?;
    }
    else {
        variable_to_cpp(writer, variable, meta_data, context)?;
        if amount < &0 {
            operator_to_cpp(writer, &ExprOperatorType::Decrement, &soul_type, span)?;
        }
        else {            
            operator_to_cpp(writer, &ExprOperatorType::Increment, &soul_type, span)?;
        }
    }

    Ok(())
}

fn deref_to_cpp(writer: &mut CppWriter, expression: &IExpression, meta_data: &MetaData, context: &CurrentContext, in_scope_id: ScopeId) -> Result<()> {
    let (expression, _) = match expression {
        IExpression::DeRef{ expression, span } => (expression, span),
        _ => return Err(new_soul_error(&token_from_span(expression.get_span()), "Internal error deref_to_cpp() called while statment is not DeRef")),
    };

    writer.push('*');
    expression_to_cpp(writer, expression, meta_data, context, in_scope_id)?;
    Ok(())
}

fn ref_to_cpp(writer: &mut CppWriter, expression: &IExpression, meta_data: &MetaData, context: &CurrentContext, in_scope_id: ScopeId) -> Result<()> {
    let (inner, span, is_const_ref) = match expression {
        IExpression::ConstRef{ expression, span } => (expression, span, true),
        IExpression::MutRef{ expression, span } => (expression, span, false),
        _ => return Err(new_soul_error(&token_from_span(expression.get_span()), "Internal error ref_to_cpp() called while statment is not MutRef or ConstRef")),
    };
    
    
    if is_const_ref {
        let type_name = get_expression_type_name(inner)?;
        let soul_type = SoulType::from_stringed_type(&type_name, &token_from_span(span), &meta_data.type_meta_data, &context.current_generics)?;
        let is_array = soul_type.is_array();

        let mut child_type = if is_array {
            soul_type.get_type_child()
                .ok_or(new_soul_error(&token_from_span(span), "Internal error could not get array child type"))?
        }
        else {
            soul_type
        };

        child_type.remove_modifier(TypeModifiers::Literal);

        if is_array {
            child_type.add_wrapper(TypeWrappers::Array)
                .map_err(|msg| new_soul_error(&token_from_span(span), &msg))?;
        }

        child_type.add_wrapper(TypeWrappers::ConstRef)
            .map_err(|msg| new_soul_error(&token_from_span(span), &msg))?;

        writer.push('(');
        writer.push_str(CppType::from_soul_type(&child_type, meta_data, context, span)?.as_str());
        writer.push(')');
    }

    writer.push('&');
    expression_to_cpp(writer, inner, meta_data, context, in_scope_id)?;
    Ok(())
}

fn literal_to_cpp(writer: &mut CppWriter, expression: &IExpression, meta_data: &MetaData, context: &CurrentContext) -> Result<()> {
    let (value, type_name, span) = match expression {
        IExpression::Literal{ value, type_name, span } => (value, type_name, span),
        _ => return Err(new_soul_error(&token_from_span(expression.get_span()), "Internal error literal_to_cpp() called while statment is not literal")),
    };

    let expression_type = SoulType::from_stringed_type(&type_name, &token_from_span(span), &meta_data.type_meta_data, &context.current_generics)?;
    
    if expression_type.is_array() {
        let mut element_type = expression_type.get_type_child()
            .ok_or(new_soul_error(&token_from_span(span), "could not get element type of array type"))?;
        element_type.remove_modifier(TypeModifiers::Literal);

        writer.push_str("__Soul_ARRAY_LiteralCtor__(");
        writer.push_str(CppType::from_soul_type(&element_type, meta_data, context, span)?.as_str());
        writer.push(',');
        writer.push_str(&value);
        writer.push(')');
    }
    else {
        writer.push_str(value);
    }
    
    Ok(())
}

fn binary_expression_to_cpp(writer: &mut CppWriter, expression: &IExpression, meta_data: &MetaData, context: &CurrentContext, in_scope_id: ScopeId) -> Result<()> {
    let (left, operator_type, right, type_name, span) = match expression {
        IExpression::BinairyExpression{ left, operator_type, right, type_name, span } => (left, operator_type, right, type_name, span),
        _ => return Err(new_soul_error(&token_from_span(expression.get_span()), "Internal error binary_expression_to_cpp() called while statment is not BinairyExpression")),
    };

    let expression_type = SoulType::from_stringed_type(&type_name, &token_from_span(span), &meta_data.type_meta_data, &context.current_generics)?;

    if is_operator_function(operator_type) {
        operator_to_cpp(writer, operator_type, &expression_type, span)?;
        writer.push('(');
        expression_to_cpp(writer, left, meta_data, context, in_scope_id)?;
        writer.push(',');
        expression_to_cpp(writer, right, meta_data, context, in_scope_id)?;
        writer.push(')');
    }
    else {        
        writer.push('(');
        expression_to_cpp(writer, left, meta_data, context, in_scope_id)?;
        writer.push(' ');
        operator_to_cpp(writer, operator_type, &expression_type, span)?;
        writer.push(' ');
        expression_to_cpp(writer, right, meta_data, context, in_scope_id)?;
        writer.push(')');
    }

    Ok(())
}

fn is_operator_function(op: &ExprOperatorType) -> bool {
    match op {
        ExprOperatorType::Not |
        ExprOperatorType::Add |
        ExprOperatorType::Sub |
        ExprOperatorType::Mul |
        ExprOperatorType::Div |
        ExprOperatorType::Equals |
        ExprOperatorType::Modulo |
        ExprOperatorType::Invalid |
        ExprOperatorType::IsBigger |
        ExprOperatorType::NotEquals |
        ExprOperatorType::IsSmaller |
        ExprOperatorType::BitWiseOr |
        ExprOperatorType::LogicalOr |
        ExprOperatorType::Increment |
        ExprOperatorType::Decrement |
        ExprOperatorType::BitWiseAnd |
        ExprOperatorType::BitWiseXor |
        ExprOperatorType::LogicalAnd |
        ExprOperatorType::IsBiggerEquals |
        ExprOperatorType::IsSmallerEquals => false,

        ExprOperatorType::Pow |
        ExprOperatorType::Root |
        ExprOperatorType::Log => true,
    }
}

fn operator_to_cpp(writer: &mut CppWriter, op: &ExprOperatorType, for_type: &SoulType, span: &SoulSpan) -> Result<()> {
    match op {
        ExprOperatorType::Invalid => return Err(new_soul_error(&token_from_span(span), "Internal Error operatorType is Invalid")),
        ExprOperatorType::Not => writer.push('!'),
        ExprOperatorType::Equals => writer.push_str("=="),
        ExprOperatorType::NotEquals => writer.push_str("!="),
        ExprOperatorType::IsSmaller => writer.push('<'),
        ExprOperatorType::IsSmallerEquals => writer.push_str("<="),
        ExprOperatorType::IsBigger => writer.push('>'),
        ExprOperatorType::IsBiggerEquals => writer.push_str(">="),
        ExprOperatorType::Add => writer.push('+'),
        ExprOperatorType::Sub => writer.push('-'),
        ExprOperatorType::Mul => writer.push('*'),
        ExprOperatorType::Div => writer.push('/'),
        ExprOperatorType::Modulo => writer.push('%'),
        ExprOperatorType::BitWiseOr => writer.push('|'),
        ExprOperatorType::BitWiseAnd => writer.push('&'),
        ExprOperatorType::BitWiseXor => writer.push('^'),
        ExprOperatorType::LogicalOr =>  writer.push_str("||"),
        ExprOperatorType::LogicalAnd => writer.push_str("&&"),
        ExprOperatorType::Increment => writer.push_str("++"),
        ExprOperatorType::Decrement => writer.push_str("--"),

        ExprOperatorType::Pow => {
            if for_type.is_literal() {
                writer.push_str("__Soul_CompileConst_math__::pow");
            }
            else {
                writer.push_str("pow");
            }
        },
        ExprOperatorType::Root => {
            if for_type.is_literal() {
                writer.push_str("__Soul_CompileConst_math__::root");
            }
            else {
                writer.push_str("root");
            }
        },
        ExprOperatorType::Log => {
            if for_type.is_literal() {
                writer.push_str("__Soul_CompileConst_math__::log");
            }
            else {
                writer.push_str("log");
            }
        },
    }

    Ok(())
}

fn get_expression_type_name<'a>(expression: &'a IExpression) -> Result<&'a String> {
    match expression {
        IExpression::IVariable{this, ..} => get_ivariable_type_name(this),
        IExpression::BinairyExpression{type_name, ..} => Ok(type_name),
        IExpression::Literal{type_name, ..} => Ok(type_name),
        IExpression::ConstRef{expression, ..} => get_expression_type_name(expression),
        IExpression::MutRef{expression, ..} => get_expression_type_name(expression),
        IExpression::DeRef{expression, ..} => get_expression_type_name(expression),
        IExpression::Increment{variable, ..} => get_ivariable_type_name(variable),
        IExpression::FunctionCall{function_info, span, ..} => function_info.as_ref().return_type.as_ref().ok_or_else(|| new_soul_error(&token_from_span(span), "Internal error function call return type is none")),
        IExpression::EmptyExpression(soul_span) => Err(new_soul_error(&token_from_span(soul_span), "Internal error EmptyExpression has not type")),
    }
}

fn get_ivariable_type_name<'a>(variable: &'a IVariable) -> Result<&'a String> {
    match variable {
        IVariable::Variable{type_name, ..} => Ok(type_name),
    }
}


fn token_from_span(span: &SoulSpan) -> Token {
    Token{line_number: span.line_number, line_offset: span.line_offset, text: String::new()}
}











