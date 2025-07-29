use std::collections::BTreeMap;
use crate::steps::parser::get_expressions::parse_expression::get_expression;
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::SoulType;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{Array, ExprKind, Ident, NamedTuple, Tuple};
use crate::steps::{step_interfaces::{i_parser::{abstract_syntax_tree::expression::Expression, scope::ScopeBuilder}, i_tokenizer::TokenStream}};

pub fn try_get_expression_group(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Option<Expression>> {
    let group_i = stream.current_index();
    let collection_type = match SoulType::try_from_stream(stream, scopes) {
        Some(result) => {
            if stream.next().is_none() {
                return Err(err_out_of_bounds(group_i, stream));
            }
            
            Some(result?)
        },
        None => None,
    };

    if stream.current_text() != "[" && stream.current_text() != "(" {
        stream.go_to_index(group_i);
        return Ok(None)
    }

    let is_array = stream.current_text() == "[";

    if !is_array && (stream.peek().is_some_and(|token| token.text == ":") || stream.peek_multiple(2).is_some_and(|token| token.text == ":"))  {
        let values = parse_named_group(stream, scopes)?;
        let group_span = stream.current_span().combine(&stream[group_i].span);

        Ok(Some(Expression::new(ExprKind::NamedTuple(NamedTuple{object_type: collection_type, values}), group_span)))
    }
    else {
        // e.g List(1,2,3) is function call instead of tuple
        if !is_array && collection_type.is_some() {
            stream.go_to_index(group_i);
            return Ok(None)
        }

        let (element_type, values) = parse_group(stream, scopes)?;
        let group_span = stream.current_span().combine(&stream[group_i].span);

        if is_array {
            Ok(Some(Expression::new(ExprKind::Array(Array{collection_type, element_type, values}), group_span)))
        }
        else {
            if values.len() == 1 {
                Ok(Some(values[0].to_owned()))
            }
            else { 
                Ok(Some(Expression::new(ExprKind::Tuple(Tuple{values}), group_span)))
            }
        }
    }

}

fn parse_group(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<(Option<SoulType>, Vec<Expression>)> {
    let group_i = stream.current_index();
    let is_array = stream.current_text() == "[";
    let end_token = if is_array {"]"} else {")"};

    if stream.next().is_none() {
        return Err(err_out_of_bounds(group_i, stream));
    } 

    if stream.current_text() == "\n" {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(group_i, stream));
        } 
    }

    let element_type = match SoulType::try_from_stream(stream, scopes) {
        Some(result) => {
            stream.next();
            if !is_array {
                return Err(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), "can not put type in tuple"))
            }
            if stream.current_text() != ":" {
                return Err(new_soul_error(
                    SoulErrorKind::UnexpectedToken, 
                    stream.current_span(), 
                    format!("token: '{}' invalid after element type in array should be ':' (e.g '[int: 1,2,3,4]')", stream.current_text())
                ));
            }
            stream.next();

            Some(result?)
        },
        None => None,
    };


    let mut values = Vec::new();
    loop {
        if stream.current_text() == "\n" {

            if stream.next().is_none() {
                break;
            } 
        }

        if stream.current_text() == end_token {
            return Ok((element_type, values))
        }

        if stream.current_text() == "\n" {

            if stream.next().is_none() {
                break;
            } 
        }

        let expr = get_expression(stream, scopes, &[",", "\n", end_token])?;
        values.push(expr);

        if stream.current_text() == "\n" {

            if stream.next().is_none() {
                break;
            } 
        }
        
        if stream.current_text() == end_token {
            return Ok((element_type, values))
        }

        if stream.current_text() != "," {
            return Err(new_soul_error(
                SoulErrorKind::UnexpectedToken,
                stream.current_span(),
                format!("expected ',' or '{}' in group expression", end_token),
            ));
        }

        if stream.next().is_none() {
            break;
        } 
    }

    Err(err_out_of_bounds(group_i, stream))
}

fn parse_named_group(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<BTreeMap<Ident, Expression>> {    let group_i = stream.current_index();
    let end_token = ")";

    fn err_out_of_bounds(group_i: usize, stream: &TokenStream) -> SoulError {
        new_soul_error(SoulErrorKind::UnexpectedEnd, stream[group_i].span.combine(&stream.current_span()), "unexpeced end while parsing expression group")
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(group_i, stream));
    } 

    if stream.current_text() == "\n" {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(group_i, stream));
        } 
    }

    if stream.current_text() == ":" {
        if stream.next().is_none() {
            return Err(err_out_of_bounds(group_i, stream));
        }

        if stream.current_text() != end_token {
            return Err(new_soul_error(SoulErrorKind::UnexpectedToken, stream.current_span(), format!("token: '{}' is invalid should be ')'", end_token)))
        }

        return Ok(BTreeMap::new());
    }

    let mut values = BTreeMap::new();
    loop {
        if stream.current_text() == "\n" {

            if stream.next().is_none() {
                break;
            } 
        }

        if stream.current_text() == end_token {
            return Ok(values)
        }

        if stream.current_text() == "\n" {

            if stream.next().is_none() {
                break;
            } 
        }

        let name_i = stream.current_index();
        let name = Ident(stream.current_text().clone());
        if stream.next().is_none() {
            break;
        } 

        if stream.current_text() != ":" {
            return Err(new_soul_error(
                SoulErrorKind::InvalidType, 
                stream.current_span(), 
                "can not have a named tuple element (e.g. (field: 1, fiedl2: 1)) and unnamed tuple element (e.g. (1, 2)) in the same tuple",
            ));
        }

        if stream.next().is_none() {
            break;
        } 

        if stream.current_text() == "\n" {

            if stream.next().is_none() {
                break;
            } 
        }

        let expr = get_expression(stream, scopes, &[",", "\n", end_token])?;
        if let Some(duplicate) = values.insert(name, expr) {
            return Err(new_soul_error(SoulErrorKind::InvalidName, stream.current_span(), format!("in NamedTuple fieldName: '{}' already exists at{}:{};", stream[name_i].text, duplicate.span.line_number, duplicate.span.line_offset)))
        }

        if stream.current_text() == "\n" {

            if stream.next().is_none() {
                break;
            } 
        }
        
        if stream.current_text() == end_token {
            return Ok(values)
        }
        else if stream.current_text() != "," {
            return Err(new_soul_error(
                SoulErrorKind::UnexpectedToken,
                stream.current_span(),
                format!("expected ',' or '{}' in group expression", end_token),
            ));
        }

        if stream.next().is_none() {
            break;
        } 
    }

    Err(err_out_of_bounds(group_i, stream))
}

fn err_out_of_bounds(group_i: usize, stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream[group_i].span.combine(&stream.current_span()), "unexpeced end while parsing expression group")
}

























