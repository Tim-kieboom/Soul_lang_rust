use crate::{errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind}, soul_names::{NamesOtherKeyWords, SOUL_NAMES}, steps::{parser::{get_expressions::parse_expression::get_expression, get_statments::{parse_block::{get_block, get_block_no_scope_push}, parse_var_decl_assign::get_unwrap_var}}, step_interfaces::{i_parser::{abstract_syntax_tree::{expression::{ExprKind, Expression}, spanned::Spanned, staments::conditionals::{ElseKind, IfDecl}}, scope::{ScopeBuilder, ScopeVisibility}}, i_tokenizer::TokenStream}}};

pub fn get_if(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<IfDecl>> {
    let if_i = stream.current_index();
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }  

    scopes.push(ScopeVisibility::All);

    let condition = if stream.current_text() == SOUL_NAMES.get_name(NamesOtherKeyWords::Typeof) {
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        } 

        let var_decl = get_unwrap_var(stream, scopes)?;
        if stream.current_text() == "\n" {
            
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            } 
        }

        Expression::new(ExprKind::UnwrapVarDecl(Box::new(var_decl.node)), var_decl.span)
    }
    else {
        get_expression(stream, scopes, &["{"])?
    };

    let block = get_block_no_scope_push(ScopeVisibility::All, stream, scopes, None, vec![])?;
    scopes.pop(stream.current_span());

    let span = stream[if_i].span.combine(&stream.current_span());
    Ok(Spanned::new(IfDecl{condition, body: block.node, else_branchs: vec![]}, span))
}

pub fn get_else(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<ElseKind>> {
    let else_i = stream.current_index();
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    } 

    let else_branch = if stream.current_text() == SOUL_NAMES.get_name(NamesOtherKeyWords::If) {
        
        let mut if_decl = get_if(stream, scopes)?;
        if_decl.span = if_decl.span.combine(&stream[else_i].span);

        ElseKind::ElseIf(Box::new(if_decl))
    }
    else {
        let block = get_block(ScopeVisibility::All, stream, scopes, None, vec![])?;
        ElseKind::Else(block)
    };

    let span = stream[else_i].span.combine(&stream.current_span());
    Ok(Spanned::new(else_branch, span))
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while trying to parse if/else statment")
}














