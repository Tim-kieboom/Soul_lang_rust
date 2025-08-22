use crate::{errors::soul_error::{SoulError, SoulErrorKind}, soul_names::check_name_allow_types, steps::step_interfaces::{i_parser::{abstract_syntax_tree::{expression::Ident, soul_type::{soul_type::{Lifetime, SoulType, TypeGenericKind, TypeWrapper}, type_kind::TypeKind}}, parser_response::{new_from_stream_error, FromStreamError, FromStreamErrorKind, FromTokenStream}, scope_builder::ScopeBuilder}, i_tokenizer::TokenStream}};

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
    
    if let Err(msg) = check_name_allow_types(stream.current_text()) {
        return Err(new_from_stream_error(
            SoulErrorKind::InvalidName,
            stream.current_span(),
            msg,
            FromStreamErrorKind::IsNotOfType,
        ))
    }

    let mut collection_type = SoulType::from_type_kind(TypeKind::Unknown(Ident::new(stream.current_text())));
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

            if let Err(msg) = check_name_allow_types(name) {

                return Err(new_from_stream_error(
                    SoulErrorKind::InvalidName, 
                    stream.current_span(), 
                    msg, 
                    FromStreamErrorKind::IsNotOfType,
                ))
            }

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






























