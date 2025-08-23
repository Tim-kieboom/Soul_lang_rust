use std::collections::HashMap;

use crate::{errors::soul_error::{SoulError, SoulErrorKind}, soul_names::{check_name, check_name_allow_types, NamesInternalType, SOUL_NAMES}, steps::step_interfaces::{i_parser::{abstract_syntax_tree::{expression::Ident, soul_type::{soul_type::{Lifetime, Modifier, SoulType, TypeGenericKind, TypeWrapper}, type_kind::TypeKind}}, parser_response::{new_from_stream_error, FromStreamError, FromStreamErrorKind, FromTokenStream}, scope_builder::ScopeBuilder}, i_tokenizer::TokenStream}};

impl FromTokenStream<SoulType> for SoulType {
    fn try_from_stream(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Option<SoulType>, SoulError> {
        let begin_i = stream.current_index();

        match inner_from_stream(stream, scopes) {
            Ok(val) => Ok(Some(val)),
            Err(err) => {
                stream.go_to_index(begin_i);
                match &err.kind {
                    FromStreamErrorKind::IsOfType => Err(err.err),
                    FromStreamErrorKind::IsNotOfType => Ok(None),
                }
            },
        }
    }

    fn from_stream(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<SoulType, SoulError> {
        let begin_i = stream.current_index();
        
        match inner_from_stream(stream, scopes) {
            Ok(val) => Ok(val),
            Err(err) => {
                stream.go_to_index(begin_i);
                Err(err.err)
            },
        }
    }
}

fn inner_from_stream(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<SoulType, FromStreamError> {

    let modifier = Modifier::from_str(stream.current_text());
    if modifier != Modifier::Default {
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }
    }

    let mut collection_type = if stream.current_text() == SOUL_NAMES.get_name(NamesInternalType::None) {
        SoulType::from_type_kind(TypeKind::None)
    }
    else if stream.current_text() == "(" {
        get_tuple_type(stream, scopes)?
    }
    else if stream.peek_is("::") {
        get_double_colon_type(stream)?
    }
    else {
        check_name_allow_types(stream.current_text())
            .map_err(|msg| new_from_stream_error(SoulErrorKind::WrongType, stream.current_span(), msg, FromStreamErrorKind::IsNotOfType))?;

        SoulType::from_type_kind(TypeKind::Unknown(stream.current_text().into()))
    };

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if stream.current_text() == "<" {
        collection_type.generics = get_type_generic(stream, scopes)?;
    }

    loop {

        let wrap = TypeWrapper::from_str(stream.current_text());
        if wrap == TypeWrapper::Invalid {
            break Ok(collection_type);
        }

        collection_type.wrappers.push(wrap);
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }
    }
}

fn get_tuple_type(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<SoulType, FromStreamError> {
    debug_assert_eq!(stream.current_text(), "(");
    if stream.peek_multiple_is(2, ":") {
        return get_named_tuple_type(stream, scopes)
    }

    let mut types = vec![];
    loop {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.current_text() == ")" {
            return Ok(SoulType::from_type_kind(TypeKind::Tuple(types)))
        }

        let ty = inner_from_stream(stream, scopes)?;
        types.push(ty);


        if stream.current_text() != "," {
            break
        }
    }

    if stream.current_text() == ")" {
        Ok(SoulType::from_type_kind(TypeKind::Tuple(types)))
    }
    else {
        Err(new_from_stream_error(
            SoulErrorKind::ArgError, 
            stream.current_span(), 
            format!("token: '{}' should be ','", stream.current_text()), FromStreamErrorKind::IsNotOfType,
        ))
    }
}

fn get_named_tuple_type(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<SoulType, FromStreamError> {
    debug_assert_eq!(stream.current_text(), "(");

    let mut types = HashMap::new();
    loop {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.current_text() == ")" {
            return Ok(SoulType::from_type_kind(TypeKind::NamedTuple(types)))
        }

        check_name(stream.current_text())
            .map_err(|msg| new_from_stream_error(SoulErrorKind::InvalidName, stream.current_span(), msg, FromStreamErrorKind::IsNotOfType))?;

        let name = Ident::new(stream.current_text());

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.current_text() != ":" {
            
            return Err(new_from_stream_error(
                SoulErrorKind::InvalidType, 
                stream.current_span(), 
                format!("token: '{}', should be ':'", stream.current_text()), 
                FromStreamErrorKind::IsNotOfType,
            ))
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        let ty = inner_from_stream(stream, scopes)?;
        types.insert(name, ty);

        if stream.current_text() != "," {
            return Err(new_from_stream_error(
                SoulErrorKind::ArgError, 
                stream.current_span(), 
                format!("token: '{}' should be ','", stream.current_text()), FromStreamErrorKind::IsNotOfType,
            ))
        }
    }
}

fn get_double_colon_type(stream: &mut TokenStream) -> Result<SoulType, FromStreamError> {
    check_name_allow_types(stream.current_text())
        .map_err(|msg| new_from_stream_error(SoulErrorKind::WrongType, stream.current_span(), msg, FromStreamErrorKind::IsOfType))?;
    
    let mut base = stream.current_text().clone();
    stream.next();
    base.push_str("::");

    loop {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        check_name_allow_types(stream.current_text())
            .map_err(|msg| new_from_stream_error(SoulErrorKind::WrongType, stream.current_span(), msg, FromStreamErrorKind::IsOfType))?;

        base.push_str(stream.current_text());
        
        if !stream.peek_is("::") {
            break
        }
        base.push_str("::");
        stream.next();
    }

    Ok(SoulType::from_type_kind(TypeKind::Unknown(base.into())))
}

fn get_type_generic(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Vec<TypeGenericKind>, FromStreamError> {
    debug_assert_eq!(stream.current_text(), "<");

    let mut generics = vec![];
    loop {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.current_text() == ">" {
            break
        }

        if stream.current_text().starts_with("'") {
            let name = &stream.current_text()[1..];

            check_name_allow_types(name)
                .map_err(|msg| new_from_stream_error(SoulErrorKind::WrongType, stream.current_span(), msg, FromStreamErrorKind::IsOfType))?;


            generics.push(TypeGenericKind::Lifetime(Lifetime{name: Ident::new(stream.current_text())}));
            continue
        }

        let ty = inner_from_stream(stream, scopes)?;
        generics.push(TypeGenericKind::Type(ty));
    }

    if generics.is_empty() {
        return Err(new_from_stream_error(
            SoulErrorKind::ArgError, 
            stream.current_span(), 
            "generic is empty", 
            FromStreamErrorKind::IsOfType,
        ))
    }

    Ok(generics)
}

fn err_out_of_bounds(stream: &TokenStream) -> FromStreamError {
    new_from_stream_error(
        SoulErrorKind::UnexpectedEnd, 
        stream.current_span(), 
        "unexpeced end while parsing group expression", 
        FromStreamErrorKind::IsNotOfType,
    )
}






























