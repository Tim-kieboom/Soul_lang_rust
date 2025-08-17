use crate::soul_names::check_name;
use crate::steps::step_interfaces::i_tokenizer::TokenStream;
use crate::steps::parser::parse_generic_decl::get_generics_decl;
use crate::steps::parser::get_statments::parse_block::get_block;
use crate::steps::parser::get_expressions::parse_expression::get_expression;
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Ident;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::{spanned::Spanned};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::staments::statment::SoulThis;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::SoulType;
use crate::errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::scope::{OverloadedFunctions, ScopeBuilder, ScopeKind, ScopeVisibility};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::type_kind::{AnyRef, Modifier, TypeWrapper};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::staments::function::{ExtFnDecl, FnDecl, FnDeclKind, FunctionSignatureRef, InnerFunctionSignature, Parameter};

pub fn get_function_decl(body_calle: Option<&SoulThis>, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<FnDeclKind>> {
    let begin_i = stream.current_index();
    
    let modifier = Modifier::from_str(&stream.current_text());
    if modifier != Modifier::Default {
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    let span_calle = body_calle.map(|cal| Spanned::new(cal, stream.current_span()));
    let signature = get_function_signature(modifier, span_calle, stream, scopes)?;
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.current_text() == "\n" {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    let body = get_block(ScopeVisibility::All, stream, scopes, signature.borrow().node.calle.clone(), signature.borrow().node.params.clone())?;
    
    let span = body.span.combine(&stream[begin_i].span);
    if signature.borrow().node.calle.is_some() {
        Ok(Spanned::new(FnDeclKind::ExtFn(ExtFnDecl{signature, body: body.node}), span))
    }
    else {
        Ok(Spanned::new(FnDeclKind::Fn(FnDecl{signature, body: body.node}), span))
    }
}

pub fn get_bodyless_function_decl(body_calle: Option<&SoulThis>, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<FunctionSignatureRef>>  {
    let begin_i = stream.current_index();
    
    let modifier = Modifier::from_str(&stream.current_text());
    if modifier != Modifier::Default {
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    let span_calle = body_calle.map(|cal| Spanned::new(cal, stream.current_span()));
    let signature = get_function_signature(modifier, span_calle, stream, scopes)?; 
    
    let span = stream.current_span().combine(&stream[begin_i].span);
    Ok(Spanned::new(signature, span))
}

fn get_function_signature(modifier: Modifier, calle_body: Option<Spanned<&SoulThis>>, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<FunctionSignatureRef> {
    fn pass_err(err: SoulError, func_name: &str, stream: &TokenStream) -> SoulError {
        pass_soul_error(
            err.get_last_kind(), 
            stream.current_span(), 
            format!("while trying to get function '{}'", func_name), 
            err
        )
    }
    let begin_i = stream.current_index();
    
    let mut calle = if stream.peek().is_some_and(|token| token.text == "(") {
        calle_body.map(|this| Spanned::new(this.node.clone(), this.span))
    }
    else {
        if let Some(result) = SoulType::try_from_stream(stream, scopes) {
            let ty = result?;

            if calle_body.is_some() {
                return Err(new_soul_error(
                    SoulErrorKind::InvalidInContext, 
                    calle_body.as_ref().unwrap().span, 
                    "can not add extention type to function when in type block (e.g. in block/scope of in 'class Foo{}' function '<type> func()' not allowed because func is already automaticly 'Foo func()' )"
                ))
            }

            
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }

            Some(Spanned::new(SoulThis{ty, this: None}, stream.current_span()))
        }
        else {
            calle_body.map(|this| Spanned::new(this.node.clone(), this.span))
        }
    }; 

    let func_name_index = stream.current_index();
    let is_ctor = stream[func_name_index].text == "Ctor" || stream[func_name_index].text == "ctor";
    let mut is_array_ctor = false;
    let mut is_named_tuple_ctor = false;

    let name = if !is_ctor {
        check_name(stream.current_text())
            .map_err(|msg| new_soul_error(SoulErrorKind::InvalidName, stream.current_span(), msg))?;
    
        Ident(stream.current_text().clone())
    }
    else {
        get_ctor_name(stream, func_name_index, &mut is_array_ctor, &mut is_named_tuple_ctor)?
    };

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    const ADD_TO_SCOPE: bool = true;
    let generics = get_generics_decl(stream, scopes, ADD_TO_SCOPE)
        .map_err(|err| pass_err(err, &stream[func_name_index].text, stream))?;

    if !generics.implements.is_empty() {
        return Err(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), "inherating (e.g. 'typeof <trait>') in not allowed in function"))
    }

    let params = get_parameters(&mut calle, stream, scopes)
        .map_err(|err| pass_err(err, &stream[func_name_index].text, stream))?;
    
    if is_array_ctor {
        if params.len() != 1 {
            return Err(new_soul_error(SoulErrorKind::ArgError, stream.current_span(), "array ctor should only have 1 parameter of type '<type>[]'"))
        }

        if params[0].node.ty.wrappers.last().is_none_or(|wrap| *wrap != TypeWrapper::Array) {
            return Err(new_soul_error(SoulErrorKind::ArgError, stream.current_span(), "array ctor should have parameter of type '<type>[]'"))
        }
    }
    else if is_named_tuple_ctor {
        todo!("named tuple not impl")
    }

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
    
    if is_ctor {
        if calle.is_none() {
            return Err(new_soul_error(SoulErrorKind::ArgError, stream.current_span(), "ctor should be an extention methode (this is a contructor function because you named it 'ctor' so add type before function name to make extention type"))
        }

        if return_type.is_some() {
            return Err(new_soul_error(SoulErrorKind::ArgError, stream.current_span(), "ctor should not have a return type is by default this type"))
        }

        return_type = Some(calle.as_ref().unwrap().node.ty.clone())
    }

    let span = stream[begin_i].span.combine(&stream.current_span());
    let signature = Spanned::new(FunctionSignatureRef::new(Spanned::new(InnerFunctionSignature{name, calle, generics: generics.generics, params, return_type, modifier}, span)), span);
    check_function_with_scope(scopes, &signature)?;
    
    Ok(signature.node)
}

fn get_ctor_name(stream: &mut TokenStream, func_name_index: usize, is_array_ctor: &mut bool, is_named_tuple_ctor: &mut bool) -> Result<Ident> {
    if !stream.peek().is_some_and(|token| token.text == "::" ) {
        return Ok(Ident(stream.current_text().clone()))
    }

    if stream.next_multiple(2).is_none() {
        return Err(err_out_of_bounds(stream));
    }

    match stream.current_text().as_str() {
        "[]" => {
            *is_array_ctor = true;
        },
        "[" => {
            *is_array_ctor = true;
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }
            if stream.current_text() != "]" {
                return Err(new_soul_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), format!("token: '{}' should be ']'", stream.current_text())));
            }
        },
        "()" => {
            *is_named_tuple_ctor = true;
        },
        "(" => {
            *is_named_tuple_ctor = true;
            if stream.current_text() != ")" {
                return Err(new_soul_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), format!("token: '{}' should be ')'", stream.current_text())));
            }
        },
        _ => return Err(new_soul_error(SoulErrorKind::UnexpectedToken, stream.current_span(), format!("token: '{}' invalid valid ctor's, array: 'Ctor::[]' and namedTuple: 'Ctor::()'", stream.current_text()))),
    };

    let mut name = String::new();
    for i in func_name_index..stream.current_index()+1 {
        name.push_str(&stream[i].text);
    }

    Ok(Ident(name))
}

fn get_parameters(calle: &mut Option<Spanned<SoulThis>>, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Vec<Spanned<Parameter>>> {
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

    let mut arg_position = 0usize;
    loop {
        arg_position += 1;
        if stream.current_text() == "this" {
            convert_this(arg_position, calle, stream)?;

            if stream.current_text() == "," {
                if stream.next().is_none() {
                    return Err(err_out_of_bounds(stream));
                }
                
                continue;
            }
            else if stream.current_text() == ")" {
                break;
            }
        }

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

fn convert_this(arg_position: usize, calle: &mut Option<Spanned<SoulThis>>, stream: &mut TokenStream) -> Result<()> {
    if calle.is_none() {
        return Err(new_soul_error(SoulErrorKind::ArgError, stream.current_span(), "'this' is only allowed in extention functions (e.g. add type before function '<type> func(this, ...)' )"))
    }

    if arg_position != 1 {
        return Err(new_soul_error(SoulErrorKind::ArgError, stream.current_span(), "'this' is only allowed as first parameter (e.g. '<type> func(this, ...)' )"))
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    let any_ref = AnyRef::from_str(stream.current_text());
    let ty = if any_ref != AnyRef::Invalid {
        let mut ty = calle.as_ref().unwrap().node.ty.clone();
        match any_ref {
            AnyRef::MutRef(lifetime) => ty.wrappers.push(TypeWrapper::MutRef(lifetime)),
            AnyRef::ConstRef(lifetime) => ty.wrappers.push(TypeWrapper::ConstRef(lifetime)),
            AnyRef::Invalid => unreachable!(),
        }
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
        ty
    }
    else {
        calle.as_ref().unwrap().node.ty.clone()
    };

    calle.as_mut().unwrap().node.this = Some(ty);

    if stream.current_text() == "," || stream.current_text() == ")" {
        return Ok(());
    }
    else {
        return Err(new_soul_error(SoulErrorKind::UnexpectedToken, stream.current_span(), format!("token: '{}' is not valid atfer 'this' use '@' or '&'", stream.current_text())));
    }
}

fn check_function_with_scope<'a>(scopes: &ScopeBuilder, signature: &Spanned<FunctionSignatureRef>) -> Result<()> {
    
    let kinds = scopes.flat_lookup(&signature.node.borrow().node.name.0);
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

fn check_function(signature: &Spanned<FunctionSignatureRef>, funcs: &OverloadedFunctions) -> Result<()> {
    if funcs.borrow().iter().any(|fnc| fnc.get_signature() == &signature.node) {
        return Err(new_soul_error(
            SoulErrorKind::InvalidInContext, 
            signature.span, 
            format!(
                "function: '{}', with: '{}' already exists", 
                signature.node.borrow().node.name.0, 
                signature.node.to_string(),
            )
        ))
    }

    let ref_guard = funcs
        .borrow();

    let same_calle_fn = ref_guard
        .iter()
        .filter(|fnc| fnc.get_signature().borrow().node.calle == signature.node.borrow().node.calle)
        .last();

    if let Some(fnc) = same_calle_fn {

        if fnc.get_signature().borrow().node.return_type != signature.node.borrow().node.return_type {
            return Err(new_soul_error(
                SoulErrorKind::InvalidInContext, 
                signature.span, 
                format!(
                    "prev function of: '{}', being: '{}', used an other return type", 
                    signature.node.borrow().node.name.0,
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

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while parsing function")
}












































































