use crate::soul_names::{check_name_allow_types};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::steps::step_interfaces::i_tokenizer::TokenStream;
use crate::steps::step_interfaces::i_parser::scope::ScopeBuilder;
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::steps::parser::get_expressions::parse_expression::{get_expression};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::SoulType;
use crate::errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{Arguments, FnCall, Ident};


pub fn get_function_call(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<FnCall>> {
    fn pass_err(err: SoulError, func_name: &str, stream: &TokenStream) -> SoulError {
        pass_soul_error(
            err.get_last_kind(), 
            stream.current_span(), 
            format!("while trying to get functionCall of: '{}'", func_name), 
            err
        )
    }

    let func_name_index = stream.current_index();
    check_name_allow_types(stream.current_text())
        .map_err(|msg| new_soul_error(SoulErrorKind::InvalidName, stream.current_span(), msg))?;

    let name = Ident(stream.current_text().clone());

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    let generics = get_generics(stream, scopes)
        .map_err(|err| pass_err(err, &stream[func_name_index].text, stream))?;

    let arguments = get_arguments(stream, scopes)
        .map_err(|err| pass_err(err, &stream[func_name_index].text, stream))?;

    let span = arguments.span.combine(&generics.span).combine(&stream[func_name_index].span);
    Ok(Spanned::new(FnCall{callee: None, name, arguments: arguments.node, generics: generics.node}, span))
}

fn get_arguments(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<Vec<Arguments>>> {

    if stream.current_text() == "()" {
        return Ok(Spanned::new(vec![], stream.current_span()));
    }

    if stream.current_text() != "(" {
        return Err(new_soul_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), "function call should start with '('"))
    }

    let first_index = stream.current_index();
    let mut args: Vec<Arguments> = Vec::new();

    let mut open_bracket_stack = 1;
    while stream.next().is_some() {

        if stream.current_text() == "\n" {
        
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }
        }

        if stream.current_text() == ")" {
            open_bracket_stack -= 1;

            if open_bracket_stack <= 0 {
                let span = stream[first_index].span.combine(&stream.current_span());
                return Ok(Spanned::new(args, span));
            }
        }
        else if stream.current_text() == "(" {
            open_bracket_stack += 1;
        }

        let next = stream.peek()
            .ok_or(err_out_of_bounds(stream))?;

        let optional_name = if next.text == "=" {
            let name = Ident(stream.current_text().clone());
            if stream.next_multiple(2).is_none() {
                return Err(err_out_of_bounds(stream));
            }

            Some(name)
        }
        else {
            None
        };

        let begin_i = stream.current_index();
        let expression = get_expression(stream, scopes, &["\n", ",", ")"])
            .map_err(|child| pass_soul_error(SoulErrorKind::ArgError, stream[begin_i].span,  format!("at argument number: {}", args.len()+1), child))?;

        args.push(Arguments{name: optional_name, expression});

        if stream.current_text() == ")" {
            open_bracket_stack -= 1;

            if open_bracket_stack <= 0 {
                let span = stream[first_index].span.combine(&stream.current_span());
                return Ok(Spanned::new(args, span));
            }
        }
    } 

    Err(err_out_of_bounds(stream))
}

fn get_generics(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Result<Spanned<Vec<SoulType>>> {
    let mut generics = Vec::new();

    if stream.current_text() != "<" {
        return Ok(Spanned::new(generics, stream.current_span()));
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.current_text() == "\n" {
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    let first = stream.current_span();
    let last;
    loop {

        let ty = SoulType::from_stream(stream, scopes)
            .map_err(|child| pass_soul_error(SoulErrorKind::ArgError, stream.current_span(), "while trying to get generic", child))?;

        generics.push(ty);

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if stream.current_text() != "," {
            last = stream.current_span();
            break;
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if stream.current_text() == "\n" {
        
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }
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
    

    Ok(Spanned::new(generics, first.combine(&last)))
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while parsing FunctionCall")
}

































