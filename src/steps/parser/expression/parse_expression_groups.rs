use std::collections::HashMap;
use crate::steps::parser::expression::parse_expression::get_expression;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::function::{Constructor, FunctionCall};
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind, SoulSpan};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::{SoulType, TypeGenericKind};
use crate::steps::step_interfaces::{i_parser::scope_builder::ScopeBuilder, i_tokenizer::TokenStream};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{Array, Expression, ExpressionGroup, ExpressionKind, Ident, NamedTuple, Tuple};

pub fn try_get_expression_group(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Option<Expression>> {
    let group_i = stream.current_index();
    
    let collection_type = SoulType::try_from_stream(stream, scopes)?;

    if stream.current_text() == "()" {

        if let Some(func_ty) = collection_type {
            return tuple_to_function(func_ty, vec![], stream.current_span())
        }
        else {
            return Ok(Some(Expression::new(ExpressionKind::Default, stream.current_span())))
        }
    }
    else if stream.current_text() == "[]" {
        let array = ExpressionGroup::Array(Array{collection_type, element_type: None, values: vec![]});
        return Ok(Some(Expression::new(ExpressionKind::ExpressionGroup(array), stream.current_span())));   
    }
    else if stream.current_text() != "(" && stream.current_text() != "[" {
        stream.go_to_index(group_i);
        return Ok(None)
    }

    let is_array = stream.current_text() == "[";
    let is_named_tuple = !is_array && (stream.peek_is(":") || stream.peek_multiple_is(2, ":"));

    if is_named_tuple {
        let values = parse_named_group(stream, scopes)?;
        let span = stream[group_i].span.combine(&stream.current_span());

        if let Some(ctor_ty) = collection_type {

            Ok(Some(Expression::new(
                ExpressionKind::Constructor(Constructor{calle: ctor_ty, arguments: NamedTuple{values}}),
                span
            )))
        }
        else {
            Ok(Some(Expression::new(
                ExpressionKind::ExpressionGroup(ExpressionGroup::NamedTuple(NamedTuple{values})), 
                span,
            )))
        }
    }
    else {
        let (element_type, values) = parse_tuple_or_array(stream, scopes)?;
        let span = stream[group_i].span.combine(&stream.current_span());

        if is_array {
            Ok(Some(Expression::new(
                ExpressionKind::ExpressionGroup(ExpressionGroup::Array(Array{collection_type, element_type, values})),
                span,
            )))
        }
        else if let Some(func_ty) = collection_type {
            tuple_to_function(func_ty, values, span)
        }
        else {
            Ok(Some(Expression::new(
                ExpressionKind::ExpressionGroup(ExpressionGroup::Tuple(Tuple{values})),
                span,
            )))
        }
    }
}

fn tuple_to_function(func_ty: SoulType, values: Vec<Expression>, span: SoulSpan) -> Result<Option<Expression>> {
    let mut generics = Vec::with_capacity(func_ty.generics.len()); 
    for kind in func_ty.generics  {
        match kind {
            TypeGenericKind::Type(soul_type) => generics.push(soul_type),
            TypeGenericKind::Lifetime(_) => return Err(new_soul_error(SoulErrorKind::InvalidInContext, span, "function call can not have lifetimes in generic")),
        }
    }

    Ok(Some(Expression::new(
        ExpressionKind::FunctionCall(FunctionCall{name: func_ty.base.to_name_string().into(), callee: None, generics, arguments: Tuple{values}}),
        span
    )))
}

fn parse_named_group(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<HashMap<Ident, Expression>> {
    let group_i = stream.current_index();
    let group_end_token = ")";

    if stream.next().is_none() {
        return Err(err_out_of_bounds(group_i, stream));
    } 

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(group_i, stream))
    }

    if stream.current_text() == ":" {
        if stream.next().is_none() {
            return Err(err_out_of_bounds(group_i, stream))
        }

        if stream.current_text() != group_end_token {
            return Err(new_soul_error(
                SoulErrorKind::UnexpectedToken, 
                stream.current_span(), 
                format!("token: '{}' is invalid should be ')'", group_end_token),
            ))
        }

        return Ok(HashMap::new());
    }

    let mut values = HashMap::new();
    loop {

        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(group_i, stream))
        }

        if stream.current_text() == group_end_token {
            return Ok(values)
        }

        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(group_i, stream))
        }

        let name_i = stream.current_index();
        let name = Ident::new(stream.current_text());
        if stream.next().is_none() {
            return Err(err_out_of_bounds(group_i, stream))
        }

        if stream.current_text() != ":" {

            return Err(new_soul_error(
                SoulErrorKind::InvalidType,
                stream.current_span(),
                "can not have a named tuple element (e.g. (field: 1, fiedl2: 1)) and unnamed tuple element (e.g. (1, 2)) in the same tuple",
            ))
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(group_i, stream))
        }

        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(group_i, stream))
        }

        let expression = get_expression(stream, scopes, &[",", "\n", group_end_token])?;
        if let Some(duplicate) = values.insert(name, expression) {
            return Err(new_soul_error(
                SoulErrorKind::InvalidName, 
                stream.current_span(), 
                format!("in NamedTuple fieldName: '{}' already exists at{}:{};", stream[name_i].text, duplicate.span.line_number, duplicate.span.line_offset),
            ))
        }

        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(group_i, stream))
        }

        if stream.current_text() == group_end_token {
            return Ok(values)
        }
        else if stream.current_text() != "," {

            return Err(new_soul_error(
                SoulErrorKind::UnexpectedToken, 
                stream.current_span(), 
                format!("expected ',' or '{}' in group expression", group_end_token),
            ))
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(group_i, stream))
        }
    }
}

fn parse_tuple_or_array(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<(Option<SoulType>, Vec<Expression>)> {
    let group_i = stream.current_index();
    let is_array = stream.current_text() == "[";
    let group_end_token = if is_array {"]"} else {")"};

    if stream.next().is_none() {
        return Err(err_out_of_bounds(group_i, stream))
    }

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(group_i, stream))
    }

    let element_i = stream.current_index();
    let element_type = if let Some(result) = SoulType::try_from_stream(stream, scopes)? {

        if stream.current_text() == ":" {
            if !is_array {
                return Err(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), "can not put type in tuple"))
            }
    
            stream.next();
            Some(result)
        }
        else {
            stream.go_to_index(element_i);
            None
        }
    }
    else {
        None
    };

    let mut values = vec![];
    loop {
        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(group_i, stream))
        }

        if stream.current_text() == group_end_token {
            return Ok((element_type, values))
        }

        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(group_i, stream))
        }
        
        let expression = get_expression(stream, scopes, &[",", "\n", group_end_token])?;
        values.push(expression);

        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(group_i, stream))
        }

        if stream.current_text() == group_end_token {
            return Ok((element_type, values))
        }

        if stream.current_text() != "," {

            return Err(new_soul_error(
                SoulErrorKind::UnexpectedToken, 
                stream.current_span(), 
                format!("expected ',' or '{}' in group expression", group_end_token),
            ))
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(group_i, stream))
        }
    }
}

fn err_out_of_bounds(group_i: usize, stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream[group_i].span.combine(&stream.current_span()), "unexpeced end while parsing expression group")
}



















