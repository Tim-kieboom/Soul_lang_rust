use crate::soul_names::check_name;
use crate::steps::step_interfaces::i_tokenizer::TokenStream;
use crate::steps::parser::get_statments::parse_block::get_block;
use crate::steps::parser::get_expressions::parse_expression::get_expression;
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Ident;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::SoulType;
use crate::errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::type_kind::{Modifier};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::{spanned::Spanned, statment::FnDecl};
use crate::steps::step_interfaces::i_parser::scope::{OverloadedFunctions, ScopeBuilder, ScopeKind, ScopeVisibility};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::statment::{FunctionSignature, GenericParam, Parameter, TypeConstraint};

pub fn get_function_decl(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<FnDecl>> {
    let begin_i = stream.current_index();
    
    let modifier = Modifier::from_str(&stream.current_text());
    if modifier != Modifier::Default {
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    let signature = get_function_signature(stream, scopes)?;
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    let body = get_block(ScopeVisibility::All, stream, scopes, signature.params.clone())?;

    
    let span = body.span.combine(&stream[begin_i].span);
    Ok(Spanned::new(FnDecl{signature, body: body.node, modifier}, span))
}

fn get_function_signature(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<FunctionSignature> {
    fn pass_err(err: SoulError, func_name: &str, stream: &TokenStream) -> SoulError {
        pass_soul_error(
            err.get_last_kind(), 
            stream.current_span(), 
            format!("while trying to get function '{}'", func_name), 
            err
        )
    }
    let begin_i = stream.current_index();
    
    let calle = if let Some(result) = SoulType::try_from_stream(stream, scopes) {
        let ty = result?;
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        Some(ty)
    }
    else {
        None
    }; 

    let func_name_index = stream.current_index();
    check_name(stream.current_text())
        .map_err(|msg| new_soul_error(SoulErrorKind::InvalidName, stream.current_span(), msg))?;

    let name = Ident(stream.current_text().clone());

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    let generics = get_generics(stream, scopes)
        .map_err(|err| pass_err(err, &stream[func_name_index].text, stream))?;

    let params = get_parameters(stream, scopes)
        .map_err(|err| pass_err(err, &stream[func_name_index].text, stream))?;
        
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    let mut return_type = try_get_return_type(stream, scopes)
        .map_err(|err| pass_err(err, &stream[func_name_index].text, stream))?;

    if let Some(ty) = &return_type {
        
        if ty.is_none_type() {
            return_type = None;
        }   
    }
    else {
        stream.next_multiple(-1);
    }
    

    let span = stream[begin_i].span.combine(&stream.current_span());
    let signature = Spanned::new(FunctionSignature{name, calle, generics, params, return_type }, span);
    check_function_with_scope(scopes, &signature)?;
    
    Ok(signature.node)
}

fn get_generics(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Vec<GenericParam>> {
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

fn get_parameters(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Vec<Spanned<Parameter>>> {
    let mut params = vec![];

    if stream.current_text() == "()" {
        return Ok(params);
    }
    else if stream.current_text() != "(" {
        return Err(new_soul_error(SoulErrorKind::ArgError, stream.current_span(), format!("token: '{}', should be '(' to start in function", stream.current_text())));
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    } 

    if stream.current_text() == ")" {
        return Ok(params);
    }

    loop {
        let arg_start_i = stream.current_index();
        let ty = SoulType::from_stream(stream, scopes)
            .map_err(|child| pass_soul_error(SoulErrorKind::ArgError, stream.current_span(), "while trying to get parameter", child))?;
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        check_name(stream.current_text())
            .map_err(|child| new_soul_error(SoulErrorKind::ArgError, stream.current_span(), format!("while trying to parse parameter: {}", child)))?;

        let name = Ident(stream.current_text().clone());

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        let mut value = None;
        if stream.current_text() == "=" {
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }

            value = Some(get_expression(stream, scopes, &[",", ")"])?);
        }

        params.push(Spanned::new(Parameter{name, ty, default_value: value}, stream[arg_start_i].span.combine(&stream.current_span())));

        if stream.current_text() != "," {
            break;
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    if stream.current_text() != ")" {
        return Err(new_soul_error(
            SoulErrorKind::ArgError, 
            stream.current_span(), 
            format!("while trying to get parameter, parameter should en with ')' but ends on '{}'", stream.current_text())
        ));
    }

    Ok(params)
}

fn check_function_with_scope<'a>(scopes: &ScopeBuilder, signature: &Spanned<FunctionSignature>) -> Result<()> {
    
    let kinds = scopes.flat_lookup(&signature.node.name.0);
    if kinds.is_none() {
        return Ok(());
    }

    for kind in kinds.unwrap() {

        if let ScopeKind::Functions(funcs) = kind {
            return check_function(signature, funcs);  
        } 
    }

    Ok(())
}

fn check_function(signature: &Spanned<FunctionSignature>, funcs: &OverloadedFunctions) -> Result<()> {
    if funcs.borrow().iter().any(|fnc| fnc.signature == signature.node) {
        return Err(new_soul_error(
            SoulErrorKind::InvalidInContext, 
            signature.span, 
            format!(
                "function: '{}', with: '{}' already exists", 
                signature.node.name.0, 
                signature.node.to_string(),
            )
        ))
    }

    let ref_guard = funcs
        .borrow();

    let same_calle_fn = ref_guard
        .iter()
        .filter(|fnc| fnc.signature.calle == signature.node.calle)
        .last();

    if let Some(fnc) = same_calle_fn {

        if fnc.signature.return_type != signature.node.return_type {
            return Err(new_soul_error(
                SoulErrorKind::InvalidInContext, 
                signature.span, 
                format!(
                    "prev function of: '{}', being: '{}', used an other return type", 
                    signature.node.name.0,
                    signature.node.to_string(),
                )
            ))
        }
    }

    Ok(())
}

fn try_get_return_type(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Option<SoulType>> {
    match SoulType::try_from_stream(stream, scopes) {
        Some(res) => Ok(Some(res?)),
        None => Ok(None),
    }
}

fn add_generic_type_contraints(contraints: &mut Vec<TypeConstraint>, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<()> {
    todo!("plz impl add_generic_type_contraints")
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while parsing function")
}














































































