use crate::soul_names::{check_name, NamesOtherKeyWords, SOUL_NAMES};
use crate::steps::parser::expression::parse_expression::get_expression;
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::steps::step_interfaces::i_parser::scope_builder::ScopeKind;
use crate::steps::parser::statment::parse_generics_decl::get_generics_decl;
use crate::steps::parser::statment::parse_function::{get_methode, get_methode_signature};
use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Ident;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::SoulType;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::type_kind::TypeKind;
use crate::steps::step_interfaces::{i_parser::scope_builder::ScopeBuilder, i_tokenizer::TokenStream};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::object::{Class, Field, FieldAccess, Struct, Trait, TraitSignature, Visibility};

pub fn get_struct(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<Struct>> {
    debug_assert_eq!(stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::Struct));

    let struct_i = stream.current_index();
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    check_name(&stream.current_text())
        .map_err(|msg| new_soul_error(SoulErrorKind::InvalidName, stream.current_span(), msg))?;
    
    let name: Ident = stream.current_text().into();

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let generics = get_generics_decl(stream, scopes)?;

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if stream.current_text() != "{" {
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            stream.current_span(), 
            format!("token: '{}' should be '{{'", stream.current_text()),
        ))
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let mut fields = vec![];
    loop {
        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.current_text() == "}" {
            break
        }

        if let Some(field) = get_field(stream, scopes)? {
            fields.push(field);
        }
        else {
            return Err(new_soul_error(
                SoulErrorKind::UnexpectedToken, 
                stream.current_span(), 
                format!("token: '{}' is invalid start token in struct body", stream.current_text()),
            ))
        }
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let span = stream[struct_i].span.combine(&stream.current_span());
    Ok(Spanned::new(Struct{name, generics: generics.generics, fields}, span))
}

pub fn get_class(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<Class>> {
    debug_assert_eq!(stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::Class));

    let class_i = stream.current_index();
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    check_name(&stream.current_text())
        .map_err(|msg| new_soul_error(SoulErrorKind::InvalidName, stream.current_span(), msg))?;
    
    let name: Ident = stream.current_text().into();

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let generics = get_generics_decl(stream, scopes)?;

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if stream.current_text() != "{" {
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            stream.current_span(), 
            format!("token: '{}' should be '{{'", stream.current_text()),
        ))
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let class_ty = SoulType::from_type_kind(TypeKind::Unknown(name.clone()));
    let mut fields = vec![];
    let mut methodes = vec![];
    loop {
        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.current_text() == "}" {
            break
        }

        if let Some(field) = get_field(stream, scopes)? {
            fields.push(field);
        }
        else if stream.current_text() == SOUL_NAMES.get_name(NamesOtherKeyWords::Impl) {
            todo!("todo>> impl trait in class")
        }
        else {
            if let Ok(methode) = get_methode(stream, scopes, class_ty.clone()) {
                methodes.push(methode);
            }
            else {
                return Err(new_soul_error(
                    SoulErrorKind::UnexpectedToken, 
                    stream.current_span(), 
                    format!("token: '{}' is invalid start token in struct body", stream.current_text()),
                ))
            }
        }
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let span = stream[class_i].span.combine(&stream.current_span());
    Ok(Spanned::new(Class{name, generics: generics.generics, fields, methodes}, span))
}

pub fn get_trait(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<Trait>> {
    debug_assert_eq!(stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::Trait));

    let trait_i = stream.current_index();
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    check_name(&stream.current_text())
        .map_err(|msg| new_soul_error(SoulErrorKind::InvalidName, stream.current_span(), msg))?;

    let name: Ident = stream.current_text().into();

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let generics = get_generics_decl(stream, scopes)?;

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if stream.current_text() != "{" {
        return Err(new_soul_error(SoulErrorKind::UnexpectedToken, stream.current_span(), format!("token: '{}' should be '{{'", stream.current_text())))
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let this_type = SoulType::from_type_kind(TypeKind::Trait(name.clone()));
    let mut methodes = vec![];
    loop {
        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.current_text() == "}" {
            break
        }

        let function_i = stream.current_index();
        let function_signature = get_methode_signature(stream, scopes, this_type.clone())?;
        let span = stream[function_i].span.combine(&stream.current_span());
        
        methodes.push(Spanned::new(function_signature, span));
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let trait_decl = Trait{
        signature: TraitSignature{name: name.clone(), generics: generics.generics, implements: generics.implements}, 
        methodes
    };
    
    let span = stream[trait_i].span.combine(&stream.current_span());
    scopes.insert(name.0.clone(), ScopeKind::Trait(trait_decl.clone()), span);
    Ok(Spanned::new(trait_decl, span))
}

fn get_field(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Option<Spanned<Field>>> {
    let field_i = stream.current_index();
    let ty = match SoulType::try_from_stream(stream, scopes)? {
        Some(val) => val,
        None => return Ok(None),
    };

    let name: Ident = stream.current_text().into();

    check_name(&stream.current_text())
        .map_err(|msg| new_soul_error(SoulErrorKind::InvalidName, stream.current_span(), msg))?;

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    const END_TOKENS: &[&str] = &["\n", ";"];
    if END_TOKENS.iter().any(|sym| sym == stream.current_text()) {
        let field = Field{
            name,
            ty,
            vis: FieldAccess::default(),
            default_value: None,
        };

        let span = stream[field_i].span.combine(&stream.current_span());
        return Ok(Some(Spanned::new(field, span)))
    }

    let vis = get_field_access(stream)?;
    let default_value = if stream.current_text() == "=" {
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        Some(get_expression(stream, scopes, END_TOKENS)?)
    }
    else {
        None
    };

    if stream.current_text() == "}" {
        stream.next_multiple(-1);
        let field = Field{ name, ty, default_value, vis};
        let span = stream[field_i].span.combine(&stream.current_span());
        Ok(Some(Spanned::new(field, span)))
    }
    else if END_TOKENS.iter().any(|sym| sym == stream.current_text()) {
        let field = Field{ name, ty, default_value, vis};
        let span = stream[field_i].span.combine(&stream.current_span());
        Ok(Some(Spanned::new(field, span)))
    }
    else {
        Err(new_soul_error(
            SoulErrorKind::InvalidInContext, 
            stream.current_span(), 
            format!("token: '{}' is invalid end of field should be enter of ';' or '}}'", stream.current_text()),
        ))
    }
}

fn get_field_access(stream: &mut TokenStream) -> Result<FieldAccess> {

    const START_TOKEN: &str = "{";
    const END_TOKEN: &str = "}";

    let mut access = FieldAccess::default();

    if stream.current_text() != START_TOKEN {
        return Ok(access);
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    loop {
        match stream.current_text().as_str() {
            "get" => {
                if access.get.is_some() {
                    return Err(new_soul_error(
                        SoulErrorKind::InvalidStringFormat, 
                        stream.current_span(), 
                        "'get' and 'Get' can not go in the same field",
                    ))
                }

                access.get = Some(Visibility::Private);
            },
            "set" => {
                if access.set.is_some() {
                    return Err(new_soul_error(
                        SoulErrorKind::InvalidStringFormat, 
                        stream.current_span(), 
                        "'get' and 'Get' can not go in the same field",
                    ))
                }

                access.set = Some(Visibility::Private);
            },
            "Get" => {
                if access.get.is_some() {
                    return Err(new_soul_error(
                        SoulErrorKind::InvalidStringFormat, 
                        stream.current_span(), 
                        "'get' and 'Get' can not go in the same field",
                    ))
                }

                access.get = Some(Visibility::Public);
            },
            "Set" => {
                if access.set.is_some() {
                    return Err(new_soul_error(
                        SoulErrorKind::InvalidStringFormat, 
                        stream.current_span(), 
                        "'get' and 'Get' can not go in the same field",
                    ))
                }

                access.set = Some(Visibility::Public);
            },
            END_TOKEN => {
                if stream.next().is_none() {
                    return Err(err_out_of_bounds(stream));
                }

                break
            },
            "\n" => {
                if stream.next().is_none() {
                    return Err(err_out_of_bounds(stream))
                }
            }
            _ => return Err(new_soul_error(
                SoulErrorKind::InvalidStringFormat, 
                stream.current_span(), 
                format!("'{}' is not allowed in field access (allowed tokens: 'get', 'Get', 'set', 'Set')", stream.current_text()),
            )),
        } 

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if stream.current_text() != "\n" {
            return Err(new_soul_error(
                SoulErrorKind::InvalidInContext, 
                stream.current_span(), 
                format!("token: '{}' invalid get/set should end on ';' or '\\n'", stream.current_text()),
            ));
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    Ok(access)
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while trying to get statments")
}






















