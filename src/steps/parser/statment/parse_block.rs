use crate::errors::soul_error::{Result, SoulError, SoulSpan};
use crate::steps::parser::statment::parse_function::get_methode;
use crate::steps::parser::statment::parse_statment::get_statment;
use crate::steps::step_interfaces::i_parser::scope_builder::{ScopeKind, Variable};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::statement::{Statement, StatementKind, UseBlock};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::SoulType;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::abstract_syntax_tree::BlockBuilder;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{Expression, ExpressionKind, VariableName};
use crate::{errors::soul_error::{new_soul_error, SoulErrorKind}, steps::step_interfaces::{i_parser::{abstract_syntax_tree::{function::{FunctionCallee, Parameter}, spanned::Spanned, statement::Block}, scope_builder::ScopeBuilder}, i_tokenizer::TokenStream}};

pub fn get_block(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    possible_this: Option<Spanned<FunctionCallee>>, 
    parameters: Vec<Spanned<Parameter>>,
) -> Result<Spanned<Block>> {
    get_inner_block(stream, scopes, possible_this, parameters, true)
}

pub fn get_block_no_scope_push(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    possible_this: Option<Spanned<FunctionCallee>>, 
    parameters: Vec<Spanned<Parameter>>,
) -> Result<Spanned<Block>> {
    get_inner_block(stream, scopes, possible_this, parameters, false)
}

pub fn get_use_block(
    stream: &mut TokenStream,
    scopes: &mut ScopeBuilder,
    this: SoulType,
    impl_trait: Option<SoulType>,
) -> Result<Spanned<UseBlock>> {
    debug_assert!(stream.current_is("{"));

    fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
        new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span_some(), format!("unexpected end while parsing use block"))
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let use_i = stream.current_index();

    scopes.push_scope();
    
    let mut block_builder = BlockBuilder::new(scopes.current_id(), stream[use_i].span);
    loop {
        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.current_text() == "}" {
            break
        }
        
        let ty = if let Some(trait_type) = &impl_trait {
            trait_type.clone()
        }
        else {
            this.clone()
        };

        if let Ok(methode) = get_methode(stream, scopes, ty) {
            block_builder.push(Statement::new(StatementKind::Function(methode.node), methode.span));
        }
        else {
            return Err(new_soul_error(
                SoulErrorKind::UnexpectedToken, 
                stream.current_span_some(), 
                format!("token: '{}' is invalid start token in struct body", stream.current_text()),
            ))
        }
    }
    
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }
    
    scopes.pop_scope(stream.current_span())?;
    
    Ok(Spanned::new(
        UseBlock{
            ty: this,
            impl_trait,
            block: block_builder.into_block().node,
        },
        stream[use_i].span.combine(&stream.current_span())
    ))
}

fn get_inner_block(
    stream: &mut TokenStream,
    scopes: &mut ScopeBuilder,
    possible_this: Option<Spanned<FunctionCallee>>,
    parameters: Vec<Spanned<Parameter>>,
    push_scope: bool,
) -> Result<Spanned<Block>> {
    if !stream.current_is("{") {
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken,
            stream.current_span_some(),
            format!("'{}' is invalid token to start block should be '{{'", stream.current_text()),
        ));
    }

    if stream.next().is_none() {
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedEnd, 
            stream.current_span_some(), 
            "unexpeced end while parsing block",
        ))
    }

    if push_scope {
        scopes.push_scope();
    }

    if let Some(this) = possible_this {
        
        if let Some(ty) = this.node.this {

            push_this(ty, scopes, stream.current_span())?;
        }
    }

    for Spanned{node: parameter, span} in parameters {
        let name = VariableName::new(parameter.name.clone(), span);
        let name_string = parameter.name.0.clone();
        let var = ScopeKind::Variable(Variable{
            name, 
            ty: parameter.ty, 
            initialize_value: Some(Expression::new(ExpressionKind::Empty, span)),
        });
        
        scopes.insert(name_string, var, span)?;
    }

    let mut block_builders = BlockBuilder::new(scopes.current_id(), stream.current_span());
    loop {
        
        if let Some(statment) = get_statment(stream, scopes)? {
            let is_end = matches!(statment.node, StatementKind::CloseBlock); 
            block_builders.push(statment);

            if is_end {
                break;
            }

            if stream.peek().is_none() {
                return Err(new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span_some(), "unexpected end while parsing block"))
            }
        }
    }

    if push_scope {
        scopes.pop_scope(stream.current_span())?;
    }
    
    Ok(block_builders.into_block())
}

fn push_this(this: SoulType, scopes: &mut ScopeBuilder, span: SoulSpan) -> Result<()> {
    
    let kind = ScopeKind::Variable(Variable{
        ty: this, 
        name: VariableName::new("this", span), 
        initialize_value: Some(Expression::new(ExpressionKind::Empty, SoulSpan::new(0,0,0)))
    });

    scopes.insert("this".into(), kind, span)
}








