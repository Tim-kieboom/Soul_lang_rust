use crate::soul_names::check_name;
use crate::steps::parser::parse_generic_decl::get_generics_decl;
use crate::steps::parser::get_statments::parse_field::try_get_field;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::type_kind::TypeKind;
use crate::steps::step_interfaces::i_parser::scope::{ScopeKind, ScopeVisibility};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Ident;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::statment::{StructDecl};
use crate::steps::step_interfaces::{i_parser::scope::ScopeBuilder, i_tokenizer::TokenStream};
use crate::errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulError, SoulErrorKind};

pub fn get_struct(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<StructDecl>> {
    
    let struct_i = stream.current_index();
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

    let generics = get_generics_decl(stream, scopes)
        .map_err(|child| pass_soul_error(SoulErrorKind::InvalidInContext, stream[struct_i].span.combine(&stream.current_span()), "while trying to get struct", child))?;

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
    
    let mut fields = Vec::new();
    loop {
        let field = match try_get_field(stream, scopes) {
            Some(result) => result?,
            None => break,
        };
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
        
        fields.push(field);
    }
    
    scopes.pop();

    if stream.current_text() != "}" {
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            stream.current_span(), 
            format!("in struct '{}' token: '{}' is unexpected (e.eg is not field or '}}')", stream[name_i].text, stream.current_text())
        ));
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    return Ok(
        Spanned::new(
            StructDecl{
                name: Ident(stream[name_i].text.clone()), 
                generics, 
                fields, 
                implements: vec![]
            },
            stream.current_span().combine(&stream[struct_i].span)
        ),
    )
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpeced end while parsing struct")
}















































