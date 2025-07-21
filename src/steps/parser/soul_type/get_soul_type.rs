use crate::steps::step_interfaces::i_parser::scope::ScopeBuilder;
use crate::steps::step_interfaces::i_tokenizer::{Token, TokenStream};
use crate::errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::type_kind::{Modifier, TypeKind, TypeWrapper};
use crate::steps::step_interfaces::i_parser::{abstract_syntax_tree::soul_type::soul_type::SoulType, parser_response::FromTokenStream};


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

    let modi = Modifier::from_str(stream.current_text());
    if modi != Modifier::Default {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    soul_type.base = get_type_kind(stream.current(), scopes)?;

    if stream.peek().is_some_and(|token| token.text == "<") {
        
        if let Err(err) = get_generic_ctor(&mut soul_type, stream, scopes) {
            return Ok(Err(err));
        }
    }

    while let Some(token) = stream.next() {
        let wrap = TypeWrapper::from_str(&token.text); 
        if wrap == TypeWrapper::Invalid {

            if stream.next_multiple(-1).is_none() {
                return Err(err_out_of_bounds(stream));
            }
            return Ok(Ok(soul_type));
        }

        soul_type.wrapper.push(wrap);
    }

    Ok(Ok(soul_type))
}

fn get_generic_ctor(soul_type: &mut SoulType, stream: &mut TokenStream, scopes: &ScopeBuilder) -> Result<()> {
    if stream.next_multiple(2).is_none() {
        return Err(err_out_of_bounds(stream));
    }

    loop {
        let ty = SoulType::from_stream(stream, scopes)
            .map_err(|child| pass_soul_error(SoulErrorKind::InvalidType, stream.current_span(), "while trying to get type in generic ctor", child))?;

        soul_type.generics.push(ty);

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


























































