use crate::errors::soul_error::{new_soul_error, Result, SoulErrorKind};
use crate::steps::parser::get_statments::parse_statment::get_statment;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::statment::StmtKind;
use crate::steps::step_interfaces::i_parser::scope::ScopeVisibility;
use crate::steps::step_interfaces::{i_parser::{abstract_syntax_tree::{spanned::Spanned, statment::Block}, scope::ScopeBuilder}, i_tokenizer::TokenStream};

pub fn get_block(scope_visability: ScopeVisibility, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<Block>> {
    
    if stream.current_text() != "{" {
        return Err(new_soul_error(SoulErrorKind::UnexpectedToken, stream.current_span(), format!("'{}' is invalid token to start block should be '{{'", stream.current_text())));
    }

    if stream.next().is_none() {
        return Err(new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpeced end while parsing block"))
    }

    scopes.push(scope_visability);
    let mut block = Spanned::new(Block{statments:vec![]}, stream.current_span());

    loop {
        
        if let Some(statment) = get_statment(stream, scopes)? {
            let is_end = matches!(statment.node, StmtKind::CloseBlock(..)); 
            block.node.statments.push(statment);

            if is_end {
                break;
            }
        }
    }

    scopes.pop();

    block.span = block.span.combine(&stream.current_span());
    Ok(block)
}












































