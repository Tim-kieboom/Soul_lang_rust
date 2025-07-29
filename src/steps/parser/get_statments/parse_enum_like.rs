use std::collections::HashSet;
use crate::soul_names::{check_name, NamesOtherKeyWords, SOUL_NAMES};
use crate::steps::step_interfaces::i_parser::scope::{ScopeVisibility};
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::literal::Literal;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Ident;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::SoulType;
use crate::errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::staments::enum_likes::{EnumVariant, TypeEnumDecl};
use crate::steps::step_interfaces::{i_parser::{abstract_syntax_tree::{spanned::Spanned, staments::enum_likes::EnumDecl}, scope::ScopeBuilder}, i_tokenizer::TokenStream};

pub fn get_type_enum(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<TypeEnumDecl>> {
        
    fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
        new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpeced end while parsing typeEnum")
    }

    let type_enum_i = stream.current_index();
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    check_name(stream.current_text())
        .map_err(|msg| new_soul_error(SoulErrorKind::InvalidName, stream.current_span(), msg))?;
    
    let name_i = stream.current_index();
    
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    } 

    let types = get_type_enum_body(stream, scopes)?; 

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    } 
    
    if stream.current_text() != ";" && stream.current_text() != "\n" {
        return Err(new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), format!("token: '{}' is incorrect end of typeEnum", stream.current_text())));
    }

    return Ok(
        Spanned::new(
            TypeEnumDecl{name: Ident(stream[name_i].text.clone()), types}, 
            stream.current_span().combine(&stream[type_enum_i].span),
        )
    );
}

pub fn get_type_enum_body(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Vec<SoulType>> {
    inner_type_enum_body(stream, scopes, true)
}

pub fn traverse_type_enum_body(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<()> {
    inner_type_enum_body(stream, scopes, true)?;
    Ok(())
}

pub fn get_enum(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<EnumDecl>> {
    
    fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
        new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpeced end while parsing enum")
    }
    
    let enum_i = stream.current_index();
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    check_name(&stream.current_text())
        .map_err(|msg| new_soul_error(SoulErrorKind::InvalidName, stream.current_span(), format!("while trying to parse enum {}", msg)))?;

    let name = Ident(stream.current_text().clone());

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.current_text() == "\n" {
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    if stream.current_text() != "{" {
        return Err(new_soul_error(SoulErrorKind::UnexpectedToken, stream.current_span(), format!("token: '{}' should be '{{'", stream.current_text())))
    }

    //this is to validate type_stack PLZ KEEP
    scopes.push(ScopeVisibility::All);

    let mut min_num = 0i64;
    let mut max_num = 0i64;
    let mut variants = Vec::new();
    let mut names = HashSet::new();
    let mut assignments = HashSet::new();
    loop {
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if stream.current_text() == "\n" {
            
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }
        }

        if stream.current_text() == "}" {
            break;
        }

        check_name(stream.current_text())
            .map_err(|msg| new_soul_error(SoulErrorKind::InvalidName, stream.current_span(), format!("while trying to parse enum variant {}", msg)))?;

        if !names.insert(stream.current_text().clone()) {
            return Err(new_soul_error(SoulErrorKind::InvalidName, stream.current_span(), format!("variant: '{}' already exist", stream.current_text())))
        }

        let name = Ident(stream.current_text().clone());

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if stream.current_text() == "\n" {
            
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }
        }
        
        let assign = get_enum_assignment(&mut assignments, stream, scopes)?;
        if let Some(num) = assign {
            min_num = min_num.min(num);
            max_num = max_num.max(num);
        }

        let value = assign.unwrap_or(variants.len() as i64);
        variants.push(EnumVariant{name, value});

        if stream.current_text() == "\n" {
            
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }
        }

        if stream.current_text() == "}" {
            break;
        }
        else if stream.current_text() != "," {
            return Err(new_soul_error(
                SoulErrorKind::UnexpectedToken, 
                stream.current_span(), 
                format!("token: '{}' is not ',' or '}}'", stream.current_text())
            ));
        }
    }

    if max_num == 0 {
        max_num = (variants.len() as i64 - 1).min(0);
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    scopes.pop();

    Ok(Spanned::new(EnumDecl{name, variants, max_num, min_num}, stream.current_span().combine(&stream[enum_i].span)))
}

fn inner_type_enum_body(stream: &mut TokenStream, scopes: &mut ScopeBuilder, return_result: bool) -> Result<Vec<SoulType>> {
    
    fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
        new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpeced end while parsing typeEnum")
    }
    
    if stream.current_text() != SOUL_NAMES.get_name(NamesOtherKeyWords::Typeof) {
        return Err(new_soul_error(SoulErrorKind::UnexpectedToken, stream.current_span(), format!("token: '{}' should be '{}'", stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::Typeof))))
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.current_text() != "[" {
        return Err(new_soul_error(SoulErrorKind::InvalidType, stream.current_span(), format!("token: '{}' is not valid to start typeEnum should start with '['", stream.current_text())))
    }

    let mut types = vec![];
    loop {
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        let ty = SoulType::from_stream(stream, scopes)
            .map_err(|child| pass_soul_error(SoulErrorKind::InvalidType, stream.current_span(), "while trying to get typeEnum", child))?;
       
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if return_result{
            types.push(ty);
        }

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
        return Err(new_soul_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), format!("token: '{}' is not valid to end typeEnum should end with ']'", stream.current_text())))
    }

    //if return_result false then yes i know i do return something but its empty (this is to avoid mallocs and so safe time when only traversing)
    Ok(types)
}

fn get_enum_assignment(assignments: &mut HashSet<i64>, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Option<i64>> {
        
    fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
        new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpeced end while parsing enum")
    }
    
    if stream.current_text() != "=" {
        return Ok(None);
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.current_text() == "\n" {
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    let is_neg = stream.current_text() == "-";
    if is_neg {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    let assign = match Literal::try_from_stream(stream, scopes) {
        Some(result) => {
            Some(result?)
        },
        None => None,
    };

    if let Some(lit) = assign {

        let mut num = match lit {
            Literal::Int(num) => num as i64,
            Literal::Uint(num) => num as i64,
            _ => return Err(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), format!("literal value of type: '{}' is not allowed as value of enum variant (only int and uint numbers)", lit.type_to_string()))),
        };

        if is_neg {
            num *= -1;
        }

        if !assignments.insert(num as i64) {
            return Err(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), format!("number: '{}' is already used (this can also be by diffrent notation so '0b1010'(binary for 10) and '10' are still duplicates)", num)))
        }
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        Ok(Some(num))
    }
    else {
        Ok(None)
    }

}






































