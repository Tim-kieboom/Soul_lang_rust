use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::type_kind::Modifier;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::staments::function::{FnDeclKind, FunctionSignatureRef};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::staments::statment::SoulThis;
use crate::steps::step_interfaces::i_tokenizer::TokenStream;
use crate::steps::step_interfaces::i_parser::scope::ScopeBuilder;
use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::steps::parser::get_statments::parse_function_decl::{get_bodyless_function_decl, get_function_decl};

pub fn try_get_methode(this: &SoulThis, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Option<Result<Spanned<FnDeclKind>>> {
    let is_methode = match check_if_methode(&stream) {
        Ok(val) => val,
        Err(err) => return Some(Err(err)),
    };

    if is_methode {
        Some(get_function_decl(Some(this), stream, scopes))
    }
    else {
        None
    }
} 

pub fn try_get_methode_decl(this: &SoulThis, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Option<Result<Spanned<FunctionSignatureRef>>> {
    let is_methode = match check_if_methode(&stream) {
        Ok(val) => val,
        Err(err) => return Some(Err(err)),
    };

    if is_methode {
        Some(get_bodyless_function_decl(Some(this), stream, scopes))
    }
    else {
        None
    }
} 

fn check_if_methode(stream: &TokenStream) -> Result<bool> {
    let mut peek_i = if Modifier::from_str(&stream.current_text()) == Modifier::Default {
       1i64
    }
    else {
        2i64
    };
    
    let mut peek = stream.peek_multiple(peek_i)
        .ok_or(err_out_of_bounds(stream))?;

        
    if peek.text == "<" {
        peek_i = get_peek_after_generic(&stream, peek_i)?;
        
        peek = stream.peek_multiple(peek_i)
            .ok_or(err_out_of_bounds(stream))?;
    }

    if peek.text == "::" {
        peek_i += 1;
        peek = stream.peek_multiple(peek_i)
            .ok_or(err_out_of_bounds(stream))?;
        
        peek_i += match peek.text.as_str() {
            "[]" => {
                1
            },
            "[" => {
                peek = stream.peek_multiple(peek_i+1)
                    .ok_or(err_out_of_bounds(stream))?;
                if peek.text != "]" {
                    return Err(new_soul_error(SoulErrorKind::UnexpectedToken, stream.current_span(), format!("token: '{}' is invalid should be '[]' or '()' (e.g. 'Ctor::[]', or 'Ctor::()')", stream.current_text())))
                }

                2
            },
            "()" => {
                1
            },
            "(" => {
                peek = stream.peek_multiple(peek_i+1)
                    .ok_or(err_out_of_bounds(stream))?;
                if peek.text != ")" {
                    return Err(new_soul_error(SoulErrorKind::UnexpectedToken, stream.current_span(), format!("token: '{}' is invalid should be '[]' or '()' (e.g. 'Ctor::[]', or 'Ctor::()')", stream.current_text())))
                }

                2
            },
            _ => return Err(new_soul_error(SoulErrorKind::UnexpectedToken, stream.current_span(), format!("token: '{}' is invalid should be '[]' or '()' (e.g. 'Ctor::[]', or 'Ctor::()')", stream.current_text())))
        };

        peek = stream.peek_multiple(peek_i)
            .ok_or(err_out_of_bounds(stream))?;
    }
    
    if peek.text != "(" {
        return Ok(false);
    }

    Ok(true)
}

fn get_peek_after_generic(stream: &TokenStream, mut peek_i: i64) -> Result<i64> {
    let mut peek = stream.peek_multiple(peek_i)
        .ok_or(new_soul_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), "unexpected end while trying to get generic (generic is not closed add '>')"))?;

    if &peek.text != "<" {
        return Err(new_soul_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), "unexpected start while trying to get generic (generic is not opened add '<')"));
    }

    let mut stack = 1;

    loop {
        peek = stream.peek_multiple(peek_i)
            .ok_or(new_soul_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), "unexpected end while trying to get generic (generic is not closed add '>')"))?;

        if peek.text == "<" {
            stack += 1;
        }
        else if peek.text == ">" {
            stack -= 1;
        }

        if stack == 0 {
            peek_i += 1;
            break Ok(peek_i);
        }
    }
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while methode")
}





























