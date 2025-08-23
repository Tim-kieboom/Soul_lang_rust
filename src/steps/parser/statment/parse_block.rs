use crate::errors::soul_error::{Result, SoulSpan};
use crate::steps::parser::statment::parse_statment::get_statment;
use crate::steps::step_interfaces::i_parser::scope_builder::{ScopeKind, Variable};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::statement::StatementKind;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::SoulType;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::abstract_syntax_tree::BlockBuilder;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{Expression, ExpressionKind};
use crate::{errors::soul_error::{new_soul_error, SoulErrorKind}, steps::step_interfaces::{i_parser::{abstract_syntax_tree::{function::{FunctionCallee, Parameter}, spanned::Spanned, statement::Block}, scope_builder::ScopeBuilder}, i_tokenizer::TokenStream}};

pub fn get_block(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    possible_this: Option<Spanned<FunctionCallee>>, 
    parameters: Vec<Spanned<Parameter>>,
) -> Result<Spanned<Block>> {
    if stream.current_text() != "{" {
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken,
            stream.current_span(),
            format!("'{}' is invalid token to start block should be '{{'", stream.current_text()),
        ));
    }

    if stream.next().is_none() {
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedEnd, 
            stream.current_span(), 
            "unexpeced end while parsing block",
        ))
    }

    scopes.push();

    if let Some(this) = possible_this {
        
        if let Some(ty) = this.node.this {

            push_this(ty, scopes);
        }
    }

    for Spanned{node: parameter, span} in parameters {
        let name = parameter.name.clone();
        let name_string = parameter.name.0.clone();
        let var = ScopeKind::Variable(Variable{
            name, 
            ty: parameter.ty, 
            initialize_value: Some(Expression::new(ExpressionKind::Empty, span)),
        });
        
        scopes.insert(name_string, var);
    }

    let mut block_builders = BlockBuilder::new(stream.current_span());
    loop {
        
        if let Some(statment) = get_statment(&mut block_builders, stream, scopes)? {
            let is_end = matches!(statment.node, StatementKind::CloseBlock); 
            block_builders.push(statment);

            if is_end {
                break;
            }

            if stream.peek().is_none() {
                return Err(new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while parsing block"))
            }
        }
    }
    
    Ok(block_builders.into_block())
}

fn push_this(this: SoulType, scopes: &mut ScopeBuilder) {
    
    let kind = ScopeKind::Variable(Variable{
        ty: this, 
        name: "this".into(), 
        initialize_value: Some(Expression::new(ExpressionKind::Empty, SoulSpan::new(0,0,0)))
    });

    scopes.insert("this".into(), kind);
}








