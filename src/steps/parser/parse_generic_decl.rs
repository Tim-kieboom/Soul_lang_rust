use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind};
use crate::soul_names::check_name;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Ident;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::statment::TypeConstraint;
use crate::steps::step_interfaces::{i_parser::{abstract_syntax_tree::statment::GenericParam, scope::ScopeBuilder}, i_tokenizer::TokenStream};

pub fn get_generics_decl(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Vec<GenericParam>> {
    let mut generics = vec![];

    if stream.current_text() != "<" {
        return Ok(generics);
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    loop {

        check_name(stream.current_text())
            .map_err(|child| new_soul_error(SoulErrorKind::ArgError, stream.current_span(), format!("while trying to parse generics: {}", child)))?;
        
        let name = Ident(stream.current_text().clone());

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        let mut constraint = vec![];
        if stream.current_text() == ":" {
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }

            add_generic_type_contraints(&mut constraint, stream, scopes)?;

            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }
        }

        generics.push(GenericParam{name, constraint});
        
        if stream.current_text() != "," {
            break;
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    if stream.current_text() != ">" {
        return Err(new_soul_error(
            SoulErrorKind::UnmatchedParenthesis, 
            stream.current_span(), 
            format!("while trying to get generics, generics should en with '>' but ends on '{}'", stream.current_text())
        ));
    } 

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    Ok(generics)
}

fn add_generic_type_contraints(contraints: &mut Vec<TypeConstraint>, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<()> {
    todo!("plz impl add_generic_type_contraints")
}

fn err_out_of_bounds(stream: &mut TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpeced end while parsing generic ctor")
}























