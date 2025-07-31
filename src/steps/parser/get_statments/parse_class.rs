use crate::soul_names::check_name;
use crate::steps::parser::parse_generic_decl::get_generics_decl;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::staments::objects::{ClassDeclRef, InnerClassDecl};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::staments::statment::SoulThis;
use crate::steps::step_interfaces::i_parser::scope::ScopeVisibility;
use crate::steps::parser::get_statments::parse_field::try_get_field;
use crate::steps::parser::get_statments::parse_methode::try_get_methode;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Ident;
use crate::steps::step_interfaces::{i_parser::scope::ScopeBuilder, i_tokenizer::TokenStream};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::SoulType;
use crate::errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulError, SoulErrorKind};

pub fn get_class(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<ClassDeclRef>> {
    
    let class_i = stream.current_index();
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    } 

    let name_i = stream.current_index();
    check_name(&stream.current_text())
        .map_err(|msg| new_soul_error(SoulErrorKind::InvalidName, stream.current_span(), msg))?;

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    scopes.push(ScopeVisibility::All);

    const ADD_TO_SCOPE: bool = true;
    let generics_decl = get_generics_decl(stream, scopes, ADD_TO_SCOPE)
        .map_err(|child| pass_soul_error(SoulErrorKind::InvalidInContext, stream[class_i].span.combine(&stream.current_span()), "while trying to get struct", child))?;

    if stream.current_text() != "{" {
        return Err(new_soul_error(SoulErrorKind::UnexpectedToken, stream.current_span(), format!("token: '{}' invalid struct's body should start with '{{'", stream.current_text())))
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    } 

    if stream.current_text() == "\n" {
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        } 
    }
    
    let this_ty = SoulType::from_type_kind(scopes.lookup_type(&stream[name_i].text)
        .ok_or(new_soul_error(SoulErrorKind::InternalError, stream.current_span(), format!("Internal error trait: '{}' not found", &stream[name_i].text)))?
        .clone());

    let body_calle = SoulThis{ty: this_ty, this: None};

    let mut fields = Vec::new();
    let mut methodes = Vec::new();
    loop {
        
        let is_field = match try_get_field(stream, scopes) {
            Some(result) => {fields.push(result?); true},
            None => false,
        };

        if !is_field {

            let methode = match try_get_methode(&body_calle, stream, scopes) {
                Some(result) => result?,
                None => break,
            };

            methodes.push(methode);
        }
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }
    
    scopes.pop(stream.current_span());

    if stream.current_text() != "}" {
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            stream.current_span(), 
            format!("in struct '{}' token: '{}' is unexpected (e.eg is not field or methode or '}}')", stream[name_i].text, stream.current_text())
        ));
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    return Ok(
        Spanned::new(
            ClassDeclRef::new(InnerClassDecl{
                name: Ident(stream[name_i].text.clone()), 
                generics: generics_decl.generics, 
                fields, 
                implements: generics_decl.implements,
                methodes,
            }),
            stream.current_span().combine(&stream[class_i].span)
        ),
    )
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpeced end while parsing struct")
}
































