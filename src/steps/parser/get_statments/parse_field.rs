use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind};
use crate::soul_names::check_name;
use crate::steps::parser::get_expressions::parse_expression::get_expression;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Ident;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::SoulType;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::staments::objects::FieldDecl;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::visibility::{FieldAccess, Visibility};
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::steps::step_interfaces::i_parser::scope::ScopeBuilder;
use crate::steps::step_interfaces::i_tokenizer::TokenStream;

pub fn try_get_field(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Option<Result<FieldDecl>> {

    let ty = match SoulType::try_from_stream(stream, scopes)? {
        Ok(ty) => ty,
        Err(err) => return Some(Err(err)),
    };

    if stream.next().is_none() {
        return Some(Err(err_out_of_bounds(stream)));
    }

    let name_i = stream.current_index();
    
    if let Err(msg) = check_name(&stream.current_text()) {
        return Some(Err(
            new_soul_error(SoulErrorKind::InvalidName, stream.current_span(), msg)
        ));
    }

    if stream.next().is_none() {
        return Some(Err(err_out_of_bounds(stream)));
    }

    const END_TOKENS: &[&str] = &["\n", ";"];
    if END_TOKENS.iter().any(|sym| sym == stream.current_text()) {
        return Some(Ok(FieldDecl{name: Ident(stream[name_i].text.clone()), ty, default_value: None, vis: FieldAccess::default() }));
    } 

    let field_access = match get_field_access(stream) {
        Ok(val) => val,
        Err(err) => return Some(Err(err)),
    };

    let default_value = if stream.current_text() == "=" {
        if stream.next().is_none() {
            return Some(Err(err_out_of_bounds(stream)));
        }

        match get_expression(stream, scopes, END_TOKENS) {
            Ok(val) => Some(val),
            Err(err) => return Some(Err(err)),
        }
    }
    else {
        None
    };

    if stream.current_text() == "}" {
        stream.next_multiple(-1);
        return Some(Ok(FieldDecl{name: Ident(stream[name_i].text.clone()), ty, default_value, vis: field_access }));
    }
    else if END_TOKENS.iter().any(|sym| sym == stream.current_text()) {
        return Some(Ok(FieldDecl{name: Ident(stream[name_i].text.clone()), ty, default_value, vis: field_access }));
    } 

    Some(Err(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), format!("token: '{}' is invalid in context", stream.current_text()))))
} 

fn get_field_access(stream: &mut TokenStream) -> Result<FieldAccess> {
    
    let mut access = FieldAccess::default();

    if stream.current_text() != "[" {
        return Ok(access);
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }
    
    loop {
        match stream.current_text().as_str() {
            "get" => {
                if access.get.is_some() {
                    return Err(new_soul_error(SoulErrorKind::InvalidStringFormat, stream.current_span(), "'get' and 'Get' can not go in the same field"))
                }

                access.get = Some(Visibility::Private);
            },
            "set" => {
                if access.set.is_some() {
                    return Err(new_soul_error(SoulErrorKind::InvalidStringFormat, stream.current_span(), "'get' and 'Get' can not go in the same field"))
                }

                access.set = Some(Visibility::Private);
            },
            "Get" => {
                if access.get.is_some() {
                    return Err(new_soul_error(SoulErrorKind::InvalidStringFormat, stream.current_span(), "'get' and 'Get' can not go in the same field"))
                }

                access.get = Some(Visibility::Public);
            },
            "Set" => {
                if access.set.is_some() {
                    return Err(new_soul_error(SoulErrorKind::InvalidStringFormat, stream.current_span(), "'get' and 'Get' can not go in the same field"))
                }

                access.set = Some(Visibility::Public);
            },
            "]" => {
                if stream.next().is_none() {
                    return Err(err_out_of_bounds(stream));
                }

                break
            },
            _ => return Err(new_soul_error(SoulErrorKind::InvalidStringFormat, stream.current_span(), format!("'{}' is not allowed in field access (allowed tokens: 'get', 'Get', 'set', 'Set')", stream.current_text()))),
        } 

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if stream.current_text() != ";" {
            return Err(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), format!("token: '{}' invalid get/set should end on ';'", stream.current_text())));
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    Ok(access)
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpeced end while parsing field")
}















