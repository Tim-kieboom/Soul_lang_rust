use crate::soul_names::{check_name, NamesOtherKeyWords, SOUL_NAMES};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::enum_like::TypeEnumBody;
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::SoulType;
use crate::errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::generic::{GenericKind, GenericParameter, TypeConstraint};
use crate::steps::step_interfaces::{i_parser::{abstract_syntax_tree::expression::Ident, scope_builder::ScopeBuilder}, i_tokenizer::TokenStream};

pub struct GenericDecl {
    pub generics: Vec<GenericParameter>,
    pub implements: Vec<SoulType>,
}

pub fn get_generics_decl(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<GenericDecl> {
    let mut generics_decl = GenericDecl{generics: vec![], implements: vec![]};

    if stream.current_text() != "<" {
        return Ok(generics_decl)
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    loop {
        let is_lifetime = stream.current_text().starts_with("'");
        let qoute_less_name = if is_lifetime {
            &stream.current_text()[1..]
        }
        else {
            &stream.current_text()
        };

        check_name(qoute_less_name)
            .map_err(|child| new_soul_error(SoulErrorKind::ArgError, stream.current_span_some(), format!("while trying to parse generics: {}", child)))?;

        let name = Ident::new(stream.current_text());

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        
        let mut constraint = vec![];
        let impl_type = if stream.current_text() == SOUL_NAMES.get_name(NamesOtherKeyWords::Impl) {

            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream))
            }

            Some(SoulType::from_stream(stream, scopes)?)
        }
        else {
            None
        };
        
        if stream.current_text() == ":" {

            if impl_type.is_some() {
                return Err(new_soul_error(
                    SoulErrorKind::ArgError, 
                    stream.current_span_some(), 
                    "can not use type contraint in impl generic",
                ))
            }
            
            if is_lifetime {
                return Err(new_soul_error(
                    SoulErrorKind::ArgError, 
                    stream.current_span_some(), 
                    "can not add type contraint to lifetime (e.g. remove ': <type>')",
                ))
            }

            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream))
            }

            add_generic_type_contraints(&mut constraint, stream, scopes)?;

            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream))
            }
        }

        let default = if stream.current_text() == "=" {
            
            if impl_type.is_some() {
                return Err(new_soul_error(
                    SoulErrorKind::ArgError, 
                    stream.current_span_some(), 
                    "can not use default type in impl generic",
                ))
            }
            
            if is_lifetime {
                return Err(new_soul_error(
                    SoulErrorKind::ArgError, 
                    stream.current_span_some(), 
                    "can not add default type to lifetime (e.g. remove '= <type>')",
                ))
            }

            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream))
            }

            let ty = SoulType::from_stream(stream, scopes)?;

            Some(ty)
        }
        else {
            None
        };

        let kind = if is_lifetime {GenericKind::Lifetime} else {GenericKind::Type{impl_type, default}};
        generics_decl.generics.push(GenericParameter{name: name.clone(), constraint, kind});
        
        if stream.current_text() != "," {
            break
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }
    }

    if stream.current_text() != ">" {
        return Err(new_soul_error(
            SoulErrorKind::UnmatchedParenthesis, 
            stream.current_span_some(), 
            format!("while trying to get generics, generics should en with '>' but ends on '{}'", stream.current_text()),
        ))
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.current_text() == "\n" {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    add_impl(&mut generics_decl, stream, scopes)?;
    add_where(&mut generics_decl, stream, scopes)?;

    Ok(generics_decl)
}

fn add_where(
    generics_decl: &mut GenericDecl, 
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder,
) -> Result<()> {
    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if stream.current_text() != SOUL_NAMES.get_name(NamesOtherKeyWords::Where) { 
        return Ok(())
    }

    loop {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(stream))
        }

        let where_generic_name = stream.current_text();
        let generic = generics_decl.generics.iter_mut()
            .find(|el| el.name.0 == *where_generic_name)
            .ok_or(new_soul_error(SoulErrorKind::UnexpectedToken, stream.current_span_some(), format!("token: '{}' is invalid should be generic (e.g. 'where T: trait,')", stream.current_text())))?;
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.current_text() != ":" {
            
            return Err(new_soul_error(
                SoulErrorKind::UnexpectedToken, 
                stream.current_span_some(), 
                format!("token: '{}' should be ':'", stream.current_text()),
            ))
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        add_generic_type_contraints(&mut generic.constraint, stream, scopes)?;

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.current_text() != "," {
            break
        }
    }

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if stream.current_text() != "{" {
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            stream.current_span_some(), 
            format!("token: '{}' invalid should be '{{'", stream.current_text()),
        ))
    }

    Ok(())
}

fn add_impl(
    generics_decl: &mut GenericDecl, 
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder,
) -> Result<()> {
    
    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if stream.current_text() != SOUL_NAMES.get_name(NamesOtherKeyWords::Impl) {
        return Ok(())
    }

    loop {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(stream))
        }

        generics_decl.implements.push(SoulType::from_stream(stream, scopes)?);

        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.current_text() != "+" {
            break
        }
    }

    if stream.current_text() != "\n" && 
       stream.current_text() != "{" &&
       stream.current_text() != SOUL_NAMES.get_name(NamesOtherKeyWords::Where)
    {
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedEnd, 
            stream.current_span_some(), 
            format!("token: '{}' invalid end should be '{{' or endline of '{}' ", stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::Where)),
        ))
    }

    Ok(())
}

fn add_generic_type_contraints(
    contraints: &mut Vec<TypeConstraint>, 
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder,
) -> Result<()> {
    
    loop {

        if stream.current_text() == "typeof" {
            
            let types = get_type_enum_body(stream, scopes)?;
            contraints.push(TypeConstraint::LiteralTypeEnum(types));
        }
        else if stream.current_text() == "[" {
            return Err(new_soul_error(
                SoulErrorKind::InvalidInContext, 
                stream.current_span_some(), 
                "'[' is not allowed in generic contraint, if you want a typeEnum try adding 'typeof' before '[' ('<T: [u8, i8]>' not ok, '<T: typeof[u8, i8]>' ok)"
            ))
        }
        else {
            let ty = SoulType::from_stream(stream, scopes)?;
            contraints.push(TypeConstraint::Type(ty));
        }

        if stream.current_text() != "+" {
            break
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }
    }

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if stream.current_text() != ">" && stream.current_text() != "," && stream.current_text() != "{" {
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            stream.current_span_some(), 
            format!("token: '{}' in not allowed in generic", stream.current_text()),
        ))
    }

    if stream.next_multiple(-1).is_none() {
        return Err(err_out_of_bounds(stream));
    }

    Ok(())
}

pub fn get_type_enum_body(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<TypeEnumBody> {
    
    fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
        new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span_some(), "unexpeced end while parsing typeEnum")
    }
    
    if stream.current_text() != SOUL_NAMES.get_name(NamesOtherKeyWords::Typeof) {
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            stream.current_span_some(), 
            format!("token: '{}' should be '{}'", stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::Typeof)),
        ))
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.current_text() != "[" {
        return Err(new_soul_error(SoulErrorKind::InvalidType, stream.current_span_some(), format!("token: '{}' is not valid to start typeEnum should start with '['", stream.current_text())))
    }

    let mut types = vec![];
    loop {
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        let ty = SoulType::from_stream(stream, scopes)
            .map_err(|child| pass_soul_error(SoulErrorKind::InvalidType, stream.current_span_some(), "while trying to get typeEnum", child))?;

        types.push(ty);

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
        return Err(new_soul_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span_some(), format!("token: '{}' is not valid to end typeEnum should end with ']'", stream.current_text())))
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    Ok(TypeEnumBody{types})
}


fn err_out_of_bounds(stream: &mut TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span_some(), "unexpeced end while parsing generic ctor")
}












































