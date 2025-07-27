use std::collections::HashMap;
use crate::soul_names::check_name;
use crate::steps::step_interfaces::i_parser::scope::ScopeBuilder;
use crate::steps::step_interfaces::i_tokenizer::{Token, TokenStream};
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Ident;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::staments::statment::Lifetime;
use crate::errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::{SoulType, TypeGenericKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::type_kind::{Modifier, TypeKind, TypeWrapper};

impl FromTokenStream<SoulType> for SoulType {
    fn from_stream(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Result<SoulType> {
        let begin_index = stream.current_index();

        let result = inner_from_token_stream(stream, scopes);
        if !result.as_ref().is_ok_and(|res| res.is_ok()) {
            stream.go_to_index(begin_index);
        }

        match result {
            Ok(val) => val,
            Err(err) => Err(err),
        }
    }

    fn try_from_stream(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Option<Result<SoulType>> {
        let begin_index = stream.current_index();

        let result = inner_from_token_stream(stream, scopes);
        if result.is_err() {
            stream.go_to_index(begin_index);
        }

        match result {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }
}

fn inner_from_token_stream(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Result<Result<SoulType>> {
    let mut soul_type = SoulType::new();

    soul_type.modifier = Modifier::from_str(stream.current_text());
    if soul_type.modifier != Modifier::Default {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    if stream.current_text() == "(" {
        soul_type.base = get_tuple_type_kind(stream, scopes)?;
    }
    else {
        soul_type.base = get_type_kind(stream.current(), scopes)?;
    }

    if stream.peek().is_some_and(|token| token.text == "<") {
        
        if let Err(err) = get_generic_ctor(&mut soul_type, stream, scopes) {
            return Ok(Err(err));
        }
    }

    let mut lifetime = None;
    while let Some(token) = stream.next() {
        if token.text.starts_with("'") {
            if lifetime.is_some() {
                return Err(new_soul_error(
                    SoulErrorKind::InvalidInContext, 
                    token.span,
                    format!("lifetime: \"{}\" is placed after another lifetime can not stack lifetimes (e.g \"'a'b\" not ok, \"'a\" ok)", token.text),
                ));
            }
            
            lifetime = Some(Lifetime{name: Ident(token.text.clone())});
            continue;
        }


        let mut wrap = TypeWrapper::from_str(&token.text); 
        if lifetime.is_some() {
            match &mut wrap {
                TypeWrapper::Array |
                TypeWrapper::Invalid |
                TypeWrapper::Pointer |
                TypeWrapper::ConstPointer => {
                    return Err(new_soul_error(
                        SoulErrorKind::InvalidInContext, 
                        token.span,
                        format!("lifetime: \"{}\" is placed before typewrapper: \"{}\" lifetime can only be used in ConstRef or MutRef (e.g \"'a&\" and \"'a@\" ok but \"'a[]\" \"'a*\" not )", token.text, wrap.to_str())
                    ));
                },
                TypeWrapper::MutRef(ref_lifetime) => *ref_lifetime = lifetime,
                TypeWrapper::ConstRef(ref_lifetime) => *ref_lifetime = lifetime,
            }

            lifetime = None;
        }

        if wrap == TypeWrapper::Invalid {

            if stream.next_multiple(-1).is_none() {
                return Err(err_out_of_bounds(stream));
            }
            return Ok(Ok(soul_type));
        }

        soul_type.wrappers.push(wrap);
    }

    Ok(Ok(soul_type))
}

fn get_tuple_type_kind(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Result<TypeKind> {
    if stream.peek_multiple(2).is_some_and(|token| token.text == ":") {
        get_named_tuple(stream, scopes)
    }
    else {
        get_tuple(stream, scopes)
    }
}

fn get_tuple(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Result<TypeKind> {
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    } 

    if stream.current_text() == "\n" {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        } 
    }
    
    let mut values = Vec::new();
    loop {
        if stream.current_text() == "\n" {

            if stream.next().is_none() {
                break;
            } 
        }

        if stream.current_text() == ")" {
            return Ok(TypeKind::Tuple(values))
        }

        if stream.current_text() == "\n" {

            if stream.next().is_none() {
                break;
            } 
        }

        let ty = SoulType::from_stream(stream, scopes)?;
        values.push(ty);

        if stream.next().is_none() {
            break;
        } 

        if stream.current_text() == "\n" {

            if stream.next().is_none() {
                break;
            } 
        }
        
        if stream.current_text() == ")" {
            return Ok(TypeKind::Tuple(values))
        }
        else if stream.current_text() != "," {
            return Err(new_soul_error(
                SoulErrorKind::UnexpectedToken,
                stream.current_span(),
                "expected ',' or ')' in group expression",
            ));
        }

        if stream.next().is_none() {
            break;
        } 
    }

    Err(err_out_of_bounds(stream))
}

fn get_named_tuple(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Result<TypeKind> {
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    } 

    if stream.current_text() == "\n" {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        } 
    }
    
    let mut values = HashMap::new();
    loop {
        if stream.current_text() == "\n" {

            if stream.next().is_none() {
                break;
            } 
        }

        if stream.current_text() == ")" {
            return Ok(TypeKind::NamedTuple(values))
        }

        if stream.current_text() == "\n" {
            
            if stream.next().is_none() {
                break;
            } 
        }

        let name = Ident(stream.current_text().clone());
        check_name(&name.0)
            .map_err(|msg| new_soul_error(SoulErrorKind::InvalidName, stream.current_span(), msg))?;

        if stream.next().is_none() {
            break;
        } 

        if stream.current_text() != ":" {
            return Err(new_soul_error(
                SoulErrorKind::InvalidType, 
                stream.current_span(), 
                format!("token: '{}' should be ':'", stream.current_text())
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

        let ty = SoulType::from_stream(stream, scopes)?;
        values.insert(name, ty);
        if stream.next().is_none() {
            break;
        } 

        if stream.current_text() == "\n" {

            if stream.next().is_none() {
                break;
            } 
        }
        
        if stream.current_text() == ")" {
            return Ok(TypeKind::NamedTuple(values))
        }
        else if stream.current_text() != "," {
            return Err(new_soul_error(
                SoulErrorKind::UnexpectedToken,
                stream.current_span(),
                "expected ',' or ')' in group expression",
            ));
        }

        if stream.next().is_none() {
            break;
        } 
    }

    Err(err_out_of_bounds(stream))
}

fn get_generic_ctor(soul_type: &mut SoulType, stream: &mut TokenStream, scopes: &ScopeBuilder) -> Result<()> {
    if stream.next_multiple(2).is_none() {
        return Err(err_out_of_bounds(stream));
    }

    loop {
        let is_lifetime = stream.current_text().starts_with("'"); 
        
        let generic = if is_lifetime {
            check_name(&stream.current_text()[1..])
                .map_err(|msg| new_soul_error(SoulErrorKind::InvalidName, stream.current_span(), format!("lifetime: \"{}\" name is invalid: {}", stream.current_text(), msg)))?;
            
            let name = Ident(stream.current_text().clone());
            TypeGenericKind::Lifetime(Lifetime{name})
        }
        else {
            let ty = SoulType::from_stream(stream, scopes)
                .map_err(|child| pass_soul_error(SoulErrorKind::InvalidType, stream.current_span(), "while trying to get type in generic ctor", child))?;
            
            TypeGenericKind::Type(ty)
        };

        soul_type.generics.push(generic);

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
        
        if stream.current_text() == ">" {
            break Ok(());
        }
        else if stream.current_text() != "," {
            return Err(new_soul_error(
                SoulErrorKind::UnexpectedToken, 
                stream.current_span(), 
                format!("'{}' is invalid in template type ctor", stream.current_text())
            ));
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
        continue;
    }
}

fn get_type_kind(base: &Token, scopes: &ScopeBuilder) -> Result<TypeKind> {
    scopes.lookup_type(base.text.as_str())
        .cloned()
        .ok_or(new_soul_error(SoulErrorKind::InvalidType, base.span, format!("type not found: '{}'", base.text)))
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(
        SoulErrorKind::UnexpectedEnd, 
        stream.current_span(), 
        "unexpected end while trying to get Type"
    )
}


























































