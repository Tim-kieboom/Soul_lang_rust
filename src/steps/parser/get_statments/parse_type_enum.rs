use crate::errors::soul_error::{pass_soul_error, Result};
use crate::steps::step_interfaces::i_parser::scope::ScopeBuilder;
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::{errors::soul_error::{new_soul_error, SoulError, SoulErrorKind}, soul_names::{NamesOtherKeyWords, SOUL_NAMES}, steps::step_interfaces::{i_parser::abstract_syntax_tree::soul_type::soul_type::SoulType, i_tokenizer::TokenStream}};



pub fn get_type_enum_body(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Vec<SoulType>> {
    inner_type_enum_body(stream, scopes, true)
}

pub fn traverse_type_enum_body(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<()> {
    inner_type_enum_body(stream, scopes, true)?;
    Ok(())
}

fn inner_type_enum_body(stream: &mut TokenStream, scopes: &mut ScopeBuilder, return_result: bool) -> Result<Vec<SoulType>> {
    if stream.current_text() == SOUL_NAMES.get_name(NamesOtherKeyWords::Typeof) {

    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.current_text() != "[" {
        return Err(new_soul_error(SoulErrorKind::InvalidType, stream.current_span(), format!("token: '{}' is not valid to start typeEnum should start with '['", stream.current_text())))
    }

    let mut types = vec![];
    loop {
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        let ty = SoulType::from_stream(stream, scopes)
            .map_err(|child| pass_soul_error(SoulErrorKind::InvalidType, stream.current_span(), "while trying to get typeEnum", child))?;
       
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if return_result{
            types.push(ty);
        }

        if stream.current_text() == "\n" {
            
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }
        }

        if stream.current_text() != "," {
            break;
        }
    }

    if stream.current_text() != "]" {
        return Err(new_soul_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), format!("token: '{}' is not valid to end typeEnum should end with ']'", stream.current_text())))
    }

    //if return_result false then yes i know i do return something but its empty (this is to avoid mallocs and so safe time when only traversing)
    Ok(types)
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpeced end while parsing typeEnum")
}























