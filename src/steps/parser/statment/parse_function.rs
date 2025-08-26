use crate::soul_names::check_name;
use crate::steps::parser::statment::parse_block::get_block;
use crate::steps::parser::statment::parse_generics_decl::get_generics_decl;
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Ident;
use crate::errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::{i_parser::scope_builder::ScopeBuilder, i_tokenizer::TokenStream};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::{AnyRef, Modifier, SoulType, TypeWrapper};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::function::{Function, FunctionCallee, FunctionSignature, Parameter};


pub fn get_function(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<Function>> {
    let func_i = stream.current_index();

    let signature = get_function_signature(stream, scopes)?;

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let block = get_block(stream, scopes, signature.callee.clone(), signature.parameters.clone())?.node;

    let span = stream[func_i].span.combine(&stream.current_span());
    Ok(Spanned::new(Function{signature, block}, span))
}

fn get_function_signature(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<FunctionSignature> {
    
    fn type_is_function_name(stream: &TokenStream) -> bool {
        stream.current_text() == "("
    }

    fn pass_err(err: SoulError, func_name: &str, stream: &TokenStream) -> SoulError {
        pass_soul_error(
            err.get_last_kind(), 
            stream.current_span(), 
            format!("while trying to get function '{}'", func_name), 
            err
        )
    }

    let ruleset = Modifier::from_str(&stream.current_text());
    if ruleset != Modifier::Default {
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    let type_i = stream.current_index();
    let mut soul_type = SoulType::try_from_stream(stream, scopes)?;
    
    if soul_type.is_some() && type_is_function_name(stream) {
        stream.go_to_index(type_i);
        soul_type = None;
    }

    let mut function_name = stream.current_text().clone();
    
    if stream.peek_is("<") {
        stream.next();
    }

    let generic_decl = get_generics_decl(stream, scopes)?;
    if !generic_decl.implements.is_empty() {
        
        return Err(new_soul_error(
            SoulErrorKind::InvalidInContext, 
            stream.current_span(), 
            "inherating (e.g. 'typeof <trait>') in not allowed in function",
        ))
    }

    let generics = generic_decl.generics;
    if !generics.is_empty() {

        if stream.next_multiple(-1).is_none() {
            return Err(err_out_of_bounds(stream))
        }
    }

    let is_ctor = function_name == "Ctor" || function_name == "ctor";
    let mut is_array_ctor = false;

    if is_ctor {
        get_ctor_type(stream, &mut function_name, &mut is_array_ctor)?;
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let (parameters, this) = get_parameters(&soul_type, stream, scopes)
        .map_err(|err| pass_err(err, &function_name, stream))?;

    if is_array_ctor {
        if parameters.len() != 1 {
            return Err(new_soul_error(SoulErrorKind::ArgError, stream.current_span(), "array ctor should only have 1 parameter of type '<type>[]'"))
        }
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let mut return_type = SoulType::try_from_stream(stream, scopes)
        .map_err(|err| pass_err(err, &stream[type_i].text, stream))?;

    if let Some(ty) = &return_type {

        if ty.is_none_type() {
            return_type = None;
        }
    }
    
    if is_ctor {
        if soul_type.is_none() {
            return Err(new_soul_error(
                SoulErrorKind::ArgError, 
                stream.current_span(), 
                "ctor should be an extention methode (this is a contructor function because you named it 'ctor' so add type before function name to make extention type",
            ))
        }

        if return_type.is_some() {
            return Err(new_soul_error(
                SoulErrorKind::ArgError, 
                stream.current_span(), 
                "ctor should not have a return type is by default this type",
            ))
        }

        return_type = Some(soul_type.as_ref().unwrap().clone());
    }

    let callee = if let Some(extention_type) = soul_type {
        Some(Spanned::new(FunctionCallee{extention_type, this}, stream[type_i].span))
    }
    else {
        None
    };

    Ok(FunctionSignature{
        callee, 
        ruleset, 
        parameters, 
        return_type,
        name: function_name.into(), 
        generics, 
    })
}

fn get_parameters(
    callee: &Option<SoulType>,
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder,
) -> Result<(Vec<Spanned<Parameter>>, Option<SoulType>)> {
    let mut parameters = vec![];

    if stream.current_text() == "()" {
        return Ok((parameters, None))
    }
    else if stream.current_text() != "(" {
        return Err(new_soul_error(
            SoulErrorKind::ArgError, 
            stream.current_span(), 
            format!("token: '{}', should be '(' to start in function", stream.current_text()),
        ))
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if stream.current_text() == ")" {
        return Ok((parameters, None))
    }

    let mut this = None;
    let mut parameter_position = 0;
    loop {

        parameter_position += 1;
        if stream.current_text() == "this" {
            this = Some(get_this_type(parameter_position, callee, stream)?);

            if stream.current_text() == "," {
                if stream.next().is_none() {
                    return Err(err_out_of_bounds(stream))
                }
                
                continue
            }
            else if stream.current_text() == ")" {
                break
            }
        }

        let arg_start_i = stream.current_index();
        let ty = SoulType::from_stream(stream, scopes)
            .map_err(|child| pass_soul_error(SoulErrorKind::ArgError, stream.current_span(), format!("while trying to get parameter number {}", parameter_position), child))?;

        check_name(stream.current_text())
            .map_err(|child| new_soul_error(SoulErrorKind::ArgError, stream.current_span(), format!("while trying to parse parameter: {}", child)))?;

        let name = Ident::new(stream.current_text());

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        let span = stream[arg_start_i].span.combine(&stream.current_span());
        parameters.push(Spanned::new(Parameter{name, ty}, span));

        if stream.current_text() != "," {
            break
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }
    }

    if stream.current_text() != ")" {
        return Err(new_soul_error(
            SoulErrorKind::ArgError, 
            stream.current_span(), 
            format!("while trying to get parameter, parameter should en with ')' but ends on '{}'", stream.current_text())
        ));
    }

    Ok((parameters, this))
}

fn get_this_type(arg_position: usize, callee: &Option<SoulType>, stream: &mut TokenStream) -> Result<SoulType> {
    if callee.is_none() {
        return Err(new_soul_error(
            SoulErrorKind::ArgError, 
            stream.current_span(), 
            "'this' can only be used in extension function",
        ))
    }

    if arg_position != 1 {
        return Err(new_soul_error(
            SoulErrorKind::ArgError, 
            stream.current_span(), 
            "'this' is only allowed as first parameter (e.g. '<type> func(this, ...)' )",
        ))
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let any_ref = AnyRef::from_str(stream.current_text());
    let ty = if any_ref != AnyRef::Invalid {
        let mut ty = callee.as_ref()
            .unwrap()
            .clone();

        match any_ref {
            AnyRef::MutRef(lifetime) => ty.wrappers.push(TypeWrapper::MutRef(lifetime)),
            AnyRef::ConstRef(lifetime) => ty.wrappers.push(TypeWrapper::ConstRef(lifetime)),
            AnyRef::Invalid => unreachable!(),
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }
        ty
    }
    else {
        callee.as_ref().unwrap().clone()
    };

    
    if stream.current_text() == "," || stream.current_text() == ")" {
        return Ok(ty)
    }
    else {
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            stream.current_span(), 
            format!("token: '{}' is not valid atfer 'this' use '@' or '&'", stream.current_text())
        ))
    }
}

fn get_ctor_type(stream: &mut TokenStream, function_name: &mut String, is_array_ctor: &mut bool) -> Result<()> {
    if stream.peek_is("[]") {
        stream.next();
        *is_array_ctor = true
    }
    else if stream.peek_is("[") {
        *is_array_ctor = true;
        if stream.next_multiple(2).is_none() {
            return Err(err_out_of_bounds(stream))
        }   

        if stream.current_text() != "]" {
            return Err(new_soul_error(
                SoulErrorKind::UnmatchedParenthesis, 
                stream.current_span(), 
                format!("token: '{}' should be ']'", stream.current_text(),
            )))
        }
    }

    if *is_array_ctor {
        function_name.push_str("[]");
    }

    Ok(())
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while parsing function")
}







