use crate::utils::serde_multi_ref::MultiRef;
use crate::steps::parser::get_statments::parse_statment::get_statment;
use crate::errors::soul_error::{new_soul_error, Result, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::scope::{ScopeKind, ScopeVisibility};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::staments::function::Parameter;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{ExprKind, Expression};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::abstract_syntax_tree::StatmentBuilder;
use crate::steps::step_interfaces::{i_parser::{abstract_syntax_tree::{spanned::Spanned}, scope::ScopeBuilder}, i_tokenizer::TokenStream};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::staments::statment::{Block, SoulThis, StmtKind, VariableDecl, VariableRef};

pub fn get_block<'a>(scope_visability: ScopeVisibility, stream: &mut TokenStream, scopes: &mut ScopeBuilder, possible_this: Option<Spanned<SoulThis>>, params: Vec<Spanned<Parameter>>) -> Result<Spanned<Block>> {
    inner_get_block(scope_visability, stream, scopes, possible_this, params, true)
}

pub fn get_block_no_scope_push<'a>(scope_visability: ScopeVisibility, stream: &mut TokenStream, scopes: &mut ScopeBuilder, possible_this: Option<Spanned<SoulThis>>, params: Vec<Spanned<Parameter>>) -> Result<Spanned<Block>> {
    inner_get_block(scope_visability, stream, scopes, possible_this, params, false)
}

fn inner_get_block<'a>(scope_visability: ScopeVisibility, stream: &mut TokenStream, scopes: &mut ScopeBuilder, possible_this: Option<Spanned<SoulThis>>, params: Vec<Spanned<Parameter>>, push_scope: bool) -> Result<Spanned<Block>> {
    
    if stream.current_text() != "{" {
        return Err(new_soul_error(SoulErrorKind::UnexpectedToken, stream.current_span(), format!("'{}' is invalid token to start block should be '{{'", stream.current_text())));
    }

    if stream.next().is_none() {
        return Err(new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpeced end while parsing block"))
    }

    if push_scope {
        scopes.push(scope_visability);
    }

    if let Some(this) = possible_this {
        scopes.insert_this_variable(this);
    }

    for Spanned{node: param, span} in params {
        let name = param.name.0.clone();
        let var = ScopeKind::Variable(VariableRef::new(VariableDecl{
            name: param.name, 
            ty: param.ty, 
            initializer: Some(Box::new(Expression::new(ExprKind::Empty, span))),
            lit_retention: None,
        }, &mut scopes.ref_pool));

        scopes.insert(name, var);
    }

    let mut block = Spanned::new(Block{statments:vec![]}, stream.current_span());
    
    let mut scope_ref = StatmentBuilder::Block(MultiRef::new(block, &mut scopes.ref_pool));
    loop {
        
        if let Some(statment) = get_statment(&mut scope_ref, stream, scopes)? {
            let is_end = matches!(statment.node, StmtKind::CloseBlock(..)); 
            scope_ref.try_push(&mut scopes.ref_pool, statment)?;

            if is_end {
                break;
            }
        }
    }
    
    if let StatmentBuilder::Block(blk) = scope_ref {
        block = blk.borrow(&scopes.ref_pool).clone();
    }
    else { unreachable!() }

    if push_scope {
        scopes.pop(stream.current_span());
    }

    block.span = block.span.combine(&stream.current_span());
    Ok(block)
}












































