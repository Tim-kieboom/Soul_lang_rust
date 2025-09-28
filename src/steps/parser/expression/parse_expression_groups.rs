use std::result;
use std::collections::HashMap;
use crate::steps::parser::expression::parse_expression::get_expression;
use crate::steps::step_interfaces::i_parser::parser_response::{FromTokenStream};
use crate::steps::step_interfaces::i_parser::scope_builder::{ScopeKind, Variable};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::pretty_format::ToString;
use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind, SoulSpan};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::type_kind::TypeKind;
use crate::steps::step_interfaces::{i_parser::scope_builder::ScopeBuilder, i_tokenizer::TokenStream};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::function::{StructConstructor, FunctionCall};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::{SoulType, TypeWrapper};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{Array, ArrayFiller, Expression, ExpressionGroup, ExpressionKind, Ident, NamedTuple, Tuple, VariableName};

pub fn try_get_expression_group(stream: &mut TokenStream, scopes: &mut ScopeBuilder, end_tokens: &[&str]) -> Result<Option<Expression>> {
    let group_i = stream.current_index();
    
    let mut collection_type = SoulType::try_from_stream(stream, scopes)?;

    if stream.current_text() == "()" {

        let span = stream[group_i].span.combine(&stream.current_span());

        if let Some(func_ty) = collection_type {
            return Ok(Some(tuple_to_function(func_ty, vec![], span)?))
        }
        else {
            return Ok(Some(Expression::new(ExpressionKind::Default, span)))
        }
    }
    else if stream.current_text() == "[]" {
        let span = stream[group_i].span.combine(&stream.current_span());

        let array = ExpressionGroup::Array(Array{collection_type, element_type: None, values: vec![]});
        return Ok(Some(Expression::new(ExpressionKind::ExpressionGroup(array), span)));   
    }
    else if stream.current_text() != "(" && stream.current_text() != "[" && stream.current_text() != "{" {
        
        if let Some(ty) = collection_type {
            
            if let Some(TypeWrapper::Array) = ty.wrappers.last() {
                stream.go_to_index(group_i);
                collection_type = None;
            }
            else {
                match ty.base {
                    TypeKind::Tuple(_) |
                    TypeKind::NamedTuple(_) => {
                        stream.go_to_index(group_i);
                        collection_type = None;
                    },
                    _ => {
                        stream.go_to_index(group_i);
                        return Ok(None)
                    }
                }
            }
        }
        else {
            stream.go_to_index(group_i);
            return Ok(None)
        }
    }

    if stream.current_text() == "{" {
        
        if end_tokens.iter().any(|el| *el == "{") {
            return Ok(None)
        }
        
        let (values, insert_defaults) = parse_named_group(stream, scopes)?;
        let span = stream[group_i].span.combine(&stream.current_span());

        if let Some(ctor_ty) = collection_type {

            Ok(Some(Expression::new(
                ExpressionKind::StructConstructor(StructConstructor{calle: ctor_ty, arguments: NamedTuple{values, insert_defaults}}),
                span
            )))
        }
        else {
            Ok(Some(Expression::new(
                ExpressionKind::ExpressionGroup(ExpressionGroup::NamedTuple(NamedTuple{values, insert_defaults})), 
                span,
            )))
        }
    }
    else {
        Ok(Some(parse_tuple_or_array(collection_type, group_i, stream, scopes)?))
    }
}

pub fn get_function_call(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<FunctionCall>> {
    let group_i = stream.current_index();
    let collection_type = SoulType::from_stream(stream, scopes)?;

    let should_be_func = parse_tuple_or_array(Some(collection_type), group_i, stream, scopes)?;
    match should_be_func.node {
        ExpressionKind::FunctionCall(function_call) => Ok(Spanned::new(function_call, should_be_func.span)),
        _ => return Err(new_soul_error(
            SoulErrorKind::WrongType, 
            stream.current_span_some(), 
            format!("expression should be functionCall nut is {} ", should_be_func.node.get_variant_name()),
        )),
    }
} 

fn tuple_to_function(func_ty: SoulType, values: Vec<Expression>, span: SoulSpan) -> Result<Expression> {
    Ok(Expression::new(
        ExpressionKind::FunctionCall(FunctionCall{name: func_ty.base.to_name_string().into(), callee: None, generics: func_ty.generics, arguments: Tuple{values}}),
        span
    ))
}

fn parse_named_group(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<(HashMap<Ident, Expression>, bool)> {
    let group_i = stream.current_index();
    const GROUP_END_TOKEN: &str = "}";

    if stream.next().is_none() {
        return Err(err_out_of_bounds(group_i, stream));
    } 

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(group_i, stream))
    }

    if stream.current_text() == ".." {
        if stream.next().is_none() {
            return Err(err_out_of_bounds(group_i, stream))
        }

        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(group_i, stream))
        }

        if stream.current_text() != GROUP_END_TOKEN {
            return Err(new_soul_error(
                SoulErrorKind::InvalidType, 
                stream.current_span_some(), 
                "namedTuple can not be empty you could do '{..}' for namedtuple with default fields (only when namedTuple is typed)", 
            ))
        }

        return Ok((HashMap::new(), true));
    }

    let mut values = HashMap::new();
    loop {

        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(group_i, stream))
        }

        if stream.current_text() == GROUP_END_TOKEN {
            break
        }

        if stream.current_text() == ".." {
            
            if stream.next().is_none() {
                return Err(err_out_of_bounds(group_i, stream))
            }

            if stream.next_if("\n").is_none() {
                return Err(err_out_of_bounds(group_i, stream))
            }

            if stream.current_text() != GROUP_END_TOKEN {
                
                return Err(new_soul_error(
                    SoulErrorKind::UnexpectedToken,
                    stream.current_span_some(),
                    format!("token: '{}' should be ')', '..' should be last argument in namedTuple", stream.current_text()),
                ))
            }
            
            return Ok((values, true))
        }

        let name_i = stream.current_index();
        let name = Ident::new(stream.current_text());

        if !stream.peek_is(":") {

            let expression = get_expression(stream, scopes, &[",", "\n", GROUP_END_TOKEN])?;
            if let ExpressionKind::Variable(_) = &expression.node {
                values.insert(name, expression);
                match end_named_loop(stream, group_i, GROUP_END_TOKEN)? {
                    BREAK => break,
                    CONTINUE => continue,
                }
            }

            return Err(new_soul_error(
                SoulErrorKind::InvalidType,
                stream.current_span_some(),
                format!("namedTuple element '{}' does not have a name", expression.to_string()),
            ))
        }

        if stream.next_multiple(2).is_none() {
            return Err(err_out_of_bounds(group_i, stream))
        }

        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(group_i, stream))
        }

        let expression = get_expression(stream, scopes, &[",", "\n", GROUP_END_TOKEN, ")"])?;
        if let Some(duplicate) = values.insert(name, expression) {
            return Err(new_soul_error(
                SoulErrorKind::InvalidName, 
                stream.current_span_some(), 
                format!("in NamedTuple fieldName: '{}' already exists at{}:{};", stream[name_i].text, duplicate.span.line_number, duplicate.span.line_offset),
            ))
        }

        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(group_i, stream))
        }

        match end_named_loop(stream, group_i, GROUP_END_TOKEN)? {
            BREAK => break,
            CONTINUE => continue,
        }
    }

    Ok((values, false))
}

const BREAK: bool = true;
const CONTINUE: bool = false;
fn end_named_loop(stream: &mut TokenStream, group_i: usize, end_token: &str) -> Result<bool> {
    if stream.current_text() == end_token {
        return Ok(BREAK)
    }
    else if stream.current_text() != "," {

        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            stream.current_span_some(), 
            format!("expected ',' or '{}' in group expression", end_token),
        ))
    }

    if stream.next().is_none() {
        Err(err_out_of_bounds(group_i, stream))
    }
    else {
        Ok(CONTINUE)
    }
}

fn parse_tuple_or_array(mut collection_type: Option<SoulType>, group_i: usize, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Expression> {
    let is_array = stream.current_text() == "[";
    let group_end_token = if is_array {"]"} else {")"};

    if stream.next().is_none() {
        return Err(err_out_of_bounds(group_i, stream))
    }

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(group_i, stream))
    }

    let element_i = stream.current_index();
    let mut element_type = if let Some(result) = SoulType::try_from_stream(stream, scopes)? {

        if stream.current_text() == ":" {
            if !is_array {
                return Err(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span_some(), "can not put type in tuple"))
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
            return tuple_or_array_to_expression(is_array, group_i, stream, collection_type, element_type, values)
        }

        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(group_i, stream))
        }

        if stream.current_text() == "for" && is_array {
            
            match try_add_array_filler(collection_type, element_type, group_i, stream, scopes)? {
                Ok(array_filler) => {
                    let span = stream[group_i].span.combine(&stream.current_span());
                    return Ok(Expression::new(ExpressionKind::ExpressionGroup(ExpressionGroup::ArrayFiller(array_filler)), span))
                },
                //not array_filler, return ownership of types and continue
                Err((col_ty, el_ty)) => {collection_type = col_ty; element_type = el_ty;},
            }
        }
        
        let expression = get_expression(stream, scopes, &[",", "\n", group_end_token])?;
        values.push(expression);

        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(group_i, stream))
        }

        if stream.current_text() == group_end_token {
            return tuple_or_array_to_expression(is_array, group_i, stream, collection_type, element_type, values)
        }

        if stream.current_text() != "," {

            return Err(new_soul_error(
                SoulErrorKind::UnexpectedToken, 
                stream.current_span_some(), 
                format!("expected ',' or '{}' in group expression", group_end_token),
            ))
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(group_i, stream))
        }
    }
}

fn try_add_array_filler(collection_type: Option<SoulType>, element_type: Option<SoulType>, group_i: usize, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<result::Result<ArrayFiller, (Option<SoulType>, Option<SoulType>)>> {

    stream.next();

    scopes.push_scope();
    let index = if stream.peek_is("in") {
        let name = VariableName::new(stream.current_text().clone(), stream.current_span());
        let variable = Variable{
            name: name.clone(), 
            ty: SoulType::none(), 
            initialize_value: Some(Expression::new(ExpressionKind::Empty, SoulSpan::new(0,0,0))),
        };
        scopes.insert(name.name.0.clone(), ScopeKind::Variable(variable), stream.current_span())?;


        if stream.next_multiple(2).is_none() {
            return Err(err_out_of_bounds(group_i, stream));
        }

        Some(name)
    }
    else {
        None
    };

    let amount = Box::new(get_expression(stream, scopes, &["=>", "{"])?);
    if stream.current_text() == "{" {
        scopes.remove_current(stream.current_span())?;
        return Ok(Err((collection_type, element_type)));
    }

    let scope_id = scopes.current_id();
    scopes.pop_scope(stream.current_span())?;

    if stream.next().is_none() {
        return Err(err_out_of_bounds(group_i, stream));
    }

    let fill_expr = Box::new(get_expression(stream, scopes, &["]"])?);
    
    let array_filler = ArrayFiller{collection_type, element_type, amount, index, fill_expr, scope_id};
    Ok(Ok(array_filler))
}

fn tuple_or_array_to_expression(
    is_array: bool, 
    group_i: usize, 
    stream: &TokenStream, 
    collection_type: Option<SoulType>, 
    element_type: Option<SoulType>, 
    values: Vec<Expression>,
) -> Result<Expression> {
    let span = stream[group_i].span.combine(&stream.current_span());
    
    if is_array {
        Ok(Expression::new(
            ExpressionKind::ExpressionGroup(ExpressionGroup::Array(Array{collection_type, element_type, values})),
            span,
        ))
    }
    else if let Some(func_ty) = collection_type {
        tuple_to_function(func_ty, values, span)
    }
    else {
        Ok(Expression::new(
            ExpressionKind::ExpressionGroup(ExpressionGroup::Tuple(Tuple{values})),
            span,
        ))
    }
}

fn err_out_of_bounds(group_i: usize, stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, Some(stream[group_i].span.combine(&stream.current_span())), "unexpeced end while parsing expression group")
}



















 