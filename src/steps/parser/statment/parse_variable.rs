use std::any::type_name;

use crate::soul_names::{NamesOtherKeyWords, SOUL_NAMES};
use crate::steps::parser::expression::parse_expression::get_expression;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{ExpressionKind, Ident};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::statement::STATMENT_END_TOKENS;
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::{Modifier, SoulType};
use crate::steps::step_interfaces::{i_parser::{abstract_syntax_tree::spanned::Spanned, scope_builder::{ScopeBuilder, Variable}}, i_tokenizer::TokenStream};


pub fn get_variable(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<Variable>> {

    let begin_i = stream.current_index();
    let possible_type = match SoulType::try_from_stream(stream, scopes)? {
        Some(ty) => {
            if stream.current_text() == "=" || stream.current_text() == ":=" {
                stream.go_to_index(begin_i);
                None
            } 
            else {
                Some(ty)
            }
        },
        None => None,
    };

    let is_type_invered = possible_type.is_none();
    let is_let = stream.current_text() == SOUL_NAMES.get_name(NamesOtherKeyWords::Let);

    let modifier = if is_type_invered {
        let modi = Modifier::from_str(stream.current_text());
        if modi != Modifier::Default || stream.current_text() == SOUL_NAMES.get_name(NamesOtherKeyWords::Let) {

            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }
        }

        modi
    }
    else {
        possible_type.as_ref()
            .unwrap()
            .modifier
            .clone()
    };

    let name: Ident = stream.current_text().into();

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if STATMENT_END_TOKENS.iter().any(|sym| sym == stream.current_text()) {

        if scopes.is_in_global() {

            return Err(new_soul_error(
                SoulErrorKind::InvalidEscapeSequence, 
                stream.current_span(), 
                format!("global variables HAVE TO BE assigned at init, variable '{}' is not assigned", name),
            ))
        }

        let ty = if is_type_invered {
            SoulType::none()
        }
        else {
            possible_type.unwrap()
        };

        let span = stream[begin_i].span.combine(&stream.current_span());
        return Ok(Spanned::new(Variable{name, ty, initialize_value: None}, span))
    }

    if is_type_invered {

        if modifier == Modifier::Default && 
           (stream.current_text() != ":=" && !is_let)
        {

            return Err(new_soul_error(
                SoulErrorKind::UnexpectedToken, 
                stream.current_span(), 
                format!("'{}' is not allowed at end of default type invered initialize variable (use ':=')", stream.current_text())
            ));
        }

    }
    else if stream.current_text() != "=" {
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            stream.current_span(), 
            format!("'{}' is not allowed at end of initialize variable (use '=')", &stream.current().text)
        ));
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if is_type_invered {

        let begin_i = stream.current_index();
        let expression = get_expression(stream, scopes, STATMENT_END_TOKENS)
            .map_err(|err| pass_soul_error(err.get_last_kind(), stream[begin_i].span, format!("while trying to get assignment of variable: '{}'", name).as_str(), err))?;

        let ty = SoulType::none();

        let span = stream[begin_i].span.combine(&stream.current_span());
        return Ok(Spanned::new(Variable{name, ty, initialize_value: Some(expression)}, span))
    }
    else {
        let begin_i = stream.current_index();
        let expression = get_expression(stream, scopes, STATMENT_END_TOKENS)
            .map_err(|err| pass_soul_error(err.get_last_kind(), stream[begin_i].span, format!("while trying to get assignment of variable: '{}'", name).as_str(), err))?;

        let mut ty = possible_type.unwrap();
        ty.modifier = modifier;

        let span = stream[begin_i].span.combine(&stream.current_span());
        return Ok(Spanned::new(Variable{name, ty, initialize_value: Some(expression)}, span))
    }
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while trying to get initialization of variable")
}





































