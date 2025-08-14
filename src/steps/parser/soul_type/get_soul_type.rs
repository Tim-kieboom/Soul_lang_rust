use std::collections::HashMap;
use crate::soul_names::check_name;
use crate::steps::parser::get_expressions::parse_path::get_page_path;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::steps::step_interfaces::i_tokenizer::TokenStream;
use crate::steps::step_interfaces::i_parser::scope::ScopeBuilder;
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Ident;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::staments::statment::Lifetime;
use crate::errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::{SoulType, TypeGenericKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::type_kind::{ExternalPath, ExternalType, Modifier, TypeKind, TypeWrapper, UnionKind, UnionType};

pub trait FromWithPath {
    fn try_from_stream_with_path(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Option<Result<SoulType>>;
    fn from_stream_with_path(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<SoulType>;
}

impl FromTokenStream<SoulType> for SoulType {
    fn from_stream(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<SoulType> {
        let begin_index = stream.current_index();

        let result = inner_from_token_stream(stream, scopes, false);
        if !result.as_ref().is_ok_and(|res| res.is_ok()) {
            stream.go_to_index(begin_index);
        }

        match result {
            Ok(val) => val,
            Err(err) => Err(err),
        }
    }

    fn try_from_stream(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Option<Result<SoulType>> {
        let begin_index = stream.current_index();

        let result = inner_from_token_stream(stream, scopes, false);
        if result.is_err() {
            stream.go_to_index(begin_index);
        }

        match result {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }
}

impl FromWithPath for SoulType {

    fn from_stream_with_path(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<SoulType> {
        let begin_index = stream.current_index();

        let result = inner_from_token_stream(stream, scopes, true);
        if !result.as_ref().is_ok_and(|res| res.is_ok()) {
            stream.go_to_index(begin_index);
        }

        match result {
            Ok(val) => val,
            Err(err) => Err(err),
        }
    }

    fn try_from_stream_with_path(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Option<Result<SoulType>> {
        let begin_index = stream.current_index();

        let result = inner_from_token_stream(stream, scopes, true);
        if result.is_err() {
            stream.go_to_index(begin_index);
        }

        match result {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }
}

fn inner_from_token_stream(stream: &mut TokenStream, scopes: &mut ScopeBuilder, with_path: bool) -> Result<Result<SoulType>> {
    let mut soul_type = SoulType::none();

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
        soul_type.base = get_type_kind(stream, scopes, with_path)?;
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

fn get_tuple_type_kind(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<TypeKind> {
    if stream.peek_multiple(2).is_some_and(|token| token.text == ":") {
        get_named_tuple(stream, scopes)
    }
    else {
        get_tuple(stream, scopes)
    }
}

fn get_tuple(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<TypeKind> {
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

fn get_named_tuple(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<TypeKind> {
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

fn get_generic_ctor(soul_type: &mut SoulType, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<()> {
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

fn get_type_kind(stream: &mut TokenStream, scopes: &mut ScopeBuilder, with_path: bool) -> Result<TypeKind> {
    let possible_kind = scopes.lookup_type(stream.current_text())
        .cloned();
    
    if stream.peek().is_some_and(|token| token.text == "::") {
        get_union_or_page(possible_kind, stream, scopes, with_path)
    }
    else if let Some(kind) = possible_kind {
        Ok(kind)
    }
    else {
        return Err(new_soul_error(SoulErrorKind::InvalidType, stream.current_span(), format!("type not found: '{}'", stream.current_text())));
    }
}

fn get_union_or_page(possible_kind: Option<TypeKind>, stream: &mut TokenStream, scopes: &mut ScopeBuilder, with_path: bool) -> Result<TypeKind> {
    if let Some(kind) = possible_kind {
        if stream.next_multiple(2).is_none() {
            return Err(err_out_of_bounds(stream));
        }

        let variant = Ident(stream.current_text().clone());
        match kind {
            TypeKind::Union(union) => Ok(TypeKind::UnionVariant(UnionType{union: UnionKind::Union(union), variant})),
            TypeKind::ExternalType(ext_ty) => Ok(TypeKind::UnionVariant(UnionType{union: UnionKind::External(ext_ty.node), variant})),
            TypeKind::ExternalPath(Spanned{node: ExternalPath{name:_, path}, span}) => {
                get_type_kind(stream, scopes, with_path)?;
                Ok(TypeKind::ExternalType(Spanned::new(ExternalType{path, name: Ident(stream.current_text().clone())}, span)))
            },
            _ => Err(new_soul_error(SoulErrorKind::WrongType, stream.current_span(), format!("'::' only allowed for union types, type: '{}'", kind.get_variant()))),
        }
    }
    else {
        if !with_path {
            return Err(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), "type has '::' is not union and is not allowed to be path"));
        }

        let path = get_page_path(stream, scopes)?.node;
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        Ok(TypeKind::ExternalType(Spanned::new(ExternalType{name: Ident(stream.current_text().clone()), path: path.path}, stream.current_span())))
    }
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(
        SoulErrorKind::UnexpectedEnd, 
        stream.current_span(), 
        "unexpected end while trying to get Type"
    )
}


























































