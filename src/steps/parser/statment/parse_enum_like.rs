use crate::errors::soul_error::Result;
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::steps::step_interfaces::i_parser::scope_builder::ScopeKind;
use crate::{errors::soul_error::{new_soul_error, SoulError, SoulErrorKind}, soul_names::{check_name, check_name_allow_types, NamesOtherKeyWords, SOUL_NAMES}, steps::{parser::{expression::parse_expression::get_expression, statment::parse_generics_decl::get_generics_decl}, step_interfaces::{i_parser::{abstract_syntax_tree::{enum_like::{Enum, EnumVariant, EnumVariantKind, Union, UnionVariant, UnionVariantKind}, expression::{Expression, ExpressionKind}, literal::Literal, soul_type::{soul_type::SoulType, type_kind::TypeKind}, spanned::Spanned}, scope_builder::ScopeBuilder}, i_tokenizer::TokenStream}}};

pub fn get_union(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<Union>> {
    debug_assert_eq!(stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::Union));

    let union_i = stream.current_index();
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    check_name(&stream.current_text())
        .map_err(|msg| new_soul_error(SoulErrorKind::InvalidName, stream.current_span_some(), msg))?;

    let name = stream.current_text().into();
    
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    scopes.push_scope();

    let generics = get_generics_decl(stream, scopes)?;
    if !generics.implements.is_empty() {
        return Err(new_soul_error(
            SoulErrorKind::InvalidInContext,
            stream.current_span_some(),
            "union could not have impl",
        ))
    }

    if stream.current_text() != "{" {
        
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            stream.current_span_some(), 
            format!("token: '{}' should be '{{'", stream.current_text()),
        ))
    }

    let mut variants = vec![];
    loop {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.current_text() == "}" {
            break
        }

        check_name_allow_types(&stream.current_text())
            .map_err(|msg| new_soul_error(SoulErrorKind::InvalidName, stream.current_span_some(), msg))?;

        let name = stream.current_text().into();

        let variant_i = stream.current_index();
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.current_text() == "," {
            let span = stream[variant_i].span.combine(&stream.current_span());
            variants.push(Spanned::new(UnionVariant{name, field: UnionVariantKind::Tuple(vec![])}, span));

            continue
        }

        let soul_type = SoulType::from_stream(stream, scopes)?;
        let field = match soul_type.base {
            TypeKind::Tuple(tuple) => UnionVariantKind::Tuple(tuple),
            TypeKind::NamedTuple(tuple) => UnionVariantKind::NamedTuple(tuple),
            _ => return Err(new_soul_error(
                SoulErrorKind::WrongType, 
                stream.current_span_some(), 
                format!("union variant should be tuple or namedTuple not {}", soul_type.to_string()),
            )),
        };

        let span = stream[variant_i].span.combine(&stream.current_span());
        variants.push(Spanned::new(UnionVariant{name, field}, span));

        if stream.current_text() == "," {
            continue
        }
        else {
            break
        }
    }

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if stream.current_text() != "}" {
        
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            stream.current_span_some(),
            format!("token: '{}' should be '}}' or you have a missing ','", stream.current_text()) 
        ))
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    scopes.pop_scope(stream.current_span())?;
    let union_decl = Union{name, generics: generics.generics, variants};
    let span = stream[union_i].span.combine(&stream.current_span());

    scopes.insert(union_decl.name.0.clone(), ScopeKind::Union(union_decl.clone()), span);
    Ok(Spanned::new(union_decl, span))
}

pub fn get_enum(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<Enum>> {
    debug_assert_eq!(stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::Enum));

    let enum_i = stream.current_index();
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    check_name(&stream.current_text())
        .map_err(|msg| new_soul_error(SoulErrorKind::InvalidName, stream.current_span_some(), msg))?;

    let name = stream.current_text().into();
    
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let enum_type = if stream.current_text() == SOUL_NAMES.get_name(NamesOtherKeyWords::Impl) {
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        Some(SoulType::from_stream(stream, scopes)?)
    }
    else {
        None
    };

    if stream.current_text() != "{" {
        
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            stream.current_span_some(), 
            format!("token: '{}' should be '{{'", stream.current_text()),
        ))
    }

    let mut variants = if enum_type.is_none() {
        EnumVariantKind::Int(vec![])
    }
    else {
        EnumVariantKind::Expression(vec![])
    };

    scopes.push_scope();

    let mut current_number = 0i64;
    loop {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.current_text() == "}" {
            break
        }

        check_name_allow_types(&stream.current_text())
            .map_err(|msg| new_soul_error(SoulErrorKind::InvalidName, stream.current_span_some(), msg))?;

        let name = stream.current_text().into();

        if !stream.peek_is("=") {
            
            match &mut variants {
                EnumVariantKind::Int(enum_variants) => enum_variants.push(EnumVariant{name, value: current_number}),
                EnumVariantKind::Expression(enum_variants) => enum_variants.push(EnumVariant{name, value: Expression::new(ExpressionKind::Default, stream.current_span())}),
            }

            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream))
            }

            if stream.current_text() == "," {
                current_number += 1;
                continue
            }
            else {
                break
            }
        }

        if stream.next_multiple(2).is_none() {
            return Err(err_out_of_bounds(stream))
        }

        match &mut variants {
            EnumVariantKind::Int(enum_variants) => {

                let literal_i = stream.current_index();
                let num = match Literal::from_stream(stream, scopes)? {
                    Literal::Int(num) => num,
                    Literal::Uint(num) => num as i64,
                    _ => return Err(new_soul_error(
                        SoulErrorKind::WrongType, 
                        stream.current_span_some(), 
                        format!("'{}' is not a literal number", stream[literal_i].text),
                    )),
                };

                if stream.next().is_none() {
                    return Err(err_out_of_bounds(stream))
                }

                current_number = num;
                enum_variants.push(EnumVariant{name, value: num});
            },
            EnumVariantKind::Expression(enum_variants) => {
                let expression = get_expression(stream, scopes, &[",", "\n", "}"])?;
                enum_variants.push(EnumVariant{name, value: expression});
            },
        }

        if stream.current_text() == "," {
            current_number += 1;
            continue
        }
        else {
            break
        }
    }

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if stream.current_text() != "}" {
        
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            stream.current_span_some(),
            format!("token: '{}' should be '}}'", stream.current_text()) 
        ))
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    scopes.pop_scope(stream.current_span())?;
    let enum_decl = Enum{name, variants};
    let span = stream[enum_i].span.combine(&stream.current_span());

    scopes.insert(enum_decl.name.0.clone(), ScopeKind::Enum(enum_decl.clone()), span);
    Ok(Spanned::new(enum_decl, span))   
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span_some(), "unexpected end while trying to get statments")
}

































