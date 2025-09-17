use crate::steps::parser::expression::parse_expression::{get_expression};
use crate::steps::step_interfaces::i_parser::scope_builder::{ScopeKind, Variable};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::steps::parser::statment::parse_block::{get_block, get_block_no_scope_push};
use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind, SoulSpan};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::SoulType;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::statement::STATMENT_END_TOKENS;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{CaseDoKind, CaseSwitch, ElseKind, Expression, ExpressionGroup, ExpressionKind, For, If, Match, Tuple, VariableName, While};
use crate::{soul_names::{NamesOtherKeyWords, SOUL_NAMES}, steps::{parser::expression::parse_expression::ExpressionStacks, step_interfaces::{i_parser::scope_builder::ScopeBuilder, i_tokenizer::TokenStream}}};

pub fn try_get_conditional(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks,
    is_statment: bool,
) -> Result<bool> {
    
    match stream.current_text().as_str() {
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::If) => get_if(stream, scopes, stacks, is_statment)?,
        
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::ForLoop) => get_for(stream, scopes, stacks)?,
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::WhileLoop) => get_while(stream, scopes, stacks)?,
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::MatchCase) => get_match(stream, scopes, stacks)?,
        _ => return Ok(false)
    }

    Ok(true)
}

fn get_match(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks,
) -> Result<()> {
    debug_assert_eq!(stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::MatchCase));

    let match_i = stream.current_index();
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    scopes.push_scope();
    let condition = Box::new(get_expression(stream, scopes, &["\n", "{"])?);

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if !stream.current_is("{") {
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            stream.current_span_some(), 
            format!("token: '{}' should be '{{'", stream.current_text()),
        ))
    }

    let mut cases = vec![];
    loop {
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.current_is("}") {
            break
        }

        scopes.push_scope();
        let if_expr = get_expression(stream, scopes, &["=>"])?;
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        let do_fn = if stream.current_is("{") {
            let block = get_block_no_scope_push(stream, scopes, None, vec![])?;
            CaseDoKind::Block(block)
        }
        else {
            CaseDoKind::Expression(get_expression(stream, scopes, &[","])?)
        };

        scopes.pop_scope(stream.current_span())?;
        
        cases.push(CaseSwitch{if_expr, do_fn});
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    scopes.pop_scope(stream.current_span())?;
    
    let span = stream[match_i].span.combine(&stream.current_span());
    let while_decl = Expression::new(
        ExpressionKind::Match(Match{condition, cases}), 
        span,
    );

    stacks.expressions.push(while_decl);
    Ok(())
}


fn get_while(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks,
) -> Result<()> {
    debug_assert_eq!(stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::WhileLoop));

    let while_i = stream.current_index();
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    scopes.push_scope();
    let condition = if stream.current_is("{") {
        None
    }
    else {
        Some(Box::new(get_expression(stream, scopes, &["\n", "{"])?))
    };

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let block = get_block_no_scope_push(stream, scopes, None, vec![])?.node;

    scopes.pop_scope(stream.current_span())?;
    
    let span = stream[while_i].span.combine(&stream.current_span());
    let while_decl = Expression::new(
        ExpressionKind::While(While{condition, block}), 
        span,
    );

    stacks.expressions.push(while_decl);
    Ok(())
}

fn get_for(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks,
) -> Result<()> {
    debug_assert_eq!(stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::ForLoop));

    let for_i = stream.current_index();
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    scopes.push_scope();
    let first = get_expression(stream, scopes, &["in", "{", "\n"])?;
    
    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let (element, collection) = if stream.current_is("in") {
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }
        (Some(Box::new(first)), Box::new(get_expression(stream, scopes, &["{", "\n"])?))
    }
    else {
        (None, Box::new(first))
    };

    if let Some(expression) = &element {
       for_element_to_scope(stream, expression, scopes)?
    }

    let block = get_block_no_scope_push(stream, scopes, None, vec![])?.node;

    let span = stream[for_i].span.combine(&stream.current_span());
    let for_decl = Expression::new(
        ExpressionKind::For(For{element, collection, block}),
        span
    );

    scopes.pop_scope(stream.current_span())?;
    stacks.expressions.push(for_decl);
    Ok(())
}

fn for_element_to_scope(stream: &TokenStream, expression: &Expression, scopes: &mut ScopeBuilder) -> Result<()> {
    match &expression.node {
        ExpressionKind::Variable(variable_name)  => {
            variable_name_to_scope(variable_name, scopes, expression.span)
        },
        ExpressionKind::ExpressionGroup(ExpressionGroup::Tuple(Tuple{values})) => {
            for value in values {
                for_element_to_scope(stream, value, scopes)?
            }
        },
        _ => return Err(new_soul_error(
            SoulErrorKind::InvalidInContext, 
            stream.current_span_some(), 
            format!("can not use expression type '{}' for element in for loop", expression.node.get_variant_name())
        ))
    }

    Ok(())
}

fn get_if(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks,
    is_statment: bool,
) -> Result<()> {
    debug_assert_eq!(stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::If));
    
    let mut first = true;
    loop {

        match IfKind::from_stream(stream, is_statment)? {
            IfKind::If => {
                if !first {
                    return Ok(())
                }
                add_if(stream, scopes, stacks)?;
            },
            IfKind::ElseIf => add_else_if(stream, scopes, stacks)?,
            
            IfKind::Else => return add_else(stream, scopes, stacks),
            IfKind::End => return Ok(()),
        }

        first = false;
    }
}

fn add_if(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks,
) -> Result<()> {

    let if_decl = parse_if(stream, scopes)?;
    let expression = Expression::new(
        ExpressionKind::If(if_decl.node),
        if_decl.span,
    );

    stacks.expressions.push(expression);
    Ok(())
}

fn parse_if(
    stream: &mut TokenStream,
    scopes: &mut ScopeBuilder,
) -> Result<Spanned<If>> {
    
    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    debug_assert!(stream.current_is(SOUL_NAMES.get_name(NamesOtherKeyWords::If)));

    let if_i = stream.current_index();
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    scopes.push_scope();

    let condition = if stream.current_is(SOUL_NAMES.get_name(NamesOtherKeyWords::Typeof)) {
        todo!("impl if typeof")
    }
    else {
        get_expression(stream, scopes, &["{"])?
    };

    let block = get_block_no_scope_push(stream, scopes, None, vec![])?;
    scopes.pop_scope(stream.current_span())?;

    let span = stream[if_i].span.combine(&stream.current_span());
    Ok(Spanned::new(
        If{
            condition: Box::new(condition), 
            block: block.node, 
            else_branchs: vec!(),
        }, 
        span,
    ))
}

fn add_else_if(    
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks,
) -> Result<()> {
    
    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    debug_assert!(stream.current_is(SOUL_NAMES.get_name(NamesOtherKeyWords::Else)));
    
    let else_i = stream.current_index();
    let mut expression = stacks.expressions.pop()
        .ok_or(new_soul_error(
            SoulErrorKind::InvalidInContext, 
            stream.current_span_some(), 
            "can not have 'else' without 'if' statment",
        ))?;

    if let ExpressionKind::If(if_decl) = &mut expression.node {
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        let mut else_if_decl = parse_if(stream, scopes)?;
        
        let span = else_if_decl.span.combine(&stream[else_i].span);
        else_if_decl.span = span;
        let kind = Spanned::new( 
            ElseKind::ElseIf(Box::new(else_if_decl)),
            span,
        );

        if_decl.else_branchs.push(kind);
    }
    else {
        return Err(new_soul_error(
            SoulErrorKind::InvalidInContext, 
            stream.current_span_some(), 
            "can not have 'else' without 'if' statment",
        ))
    }

    stacks.expressions.push(expression);
    Ok(())
}

fn add_else(    
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks,
) -> Result<()> {
        
    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    debug_assert!(stream.current_is(SOUL_NAMES.get_name(NamesOtherKeyWords::Else)));
    
    let else_i = stream.current_index();
    let mut expression = stacks.expressions.pop()
        .ok_or(new_soul_error(
            SoulErrorKind::InvalidInContext, 
            stream.current_span_some(), 
            "can not have 'else' without 'if' statment",
        ))?;

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if let ExpressionKind::If(if_decl) = &mut expression.node {
        
        let mut block = get_block(stream, scopes, None, vec![])?;
        let span = block.span.combine(&stream[else_i].span);
        block.span = span;
        if_decl.else_branchs.push(Spanned::new(ElseKind::Else(block), span));
    }
    else {
        return Err(new_soul_error(
            SoulErrorKind::InvalidInContext, 
            stream.current_span_some(), 
            "can not have 'else' without 'if' statment",
        ))
    }

    stacks.expressions.push(expression);
    Ok(())
}

fn variable_name_to_scope(variable_name: &VariableName, scopes: &mut ScopeBuilder, span: SoulSpan) {
    let variable = ScopeKind::Variable(Variable{
        name: variable_name.name.clone(), 
        ty: SoulType::none(), 
        initialize_value: Some(Expression::new(ExpressionKind::Empty, span)),
    });
    
    scopes.insert(
        variable_name.name.0.clone(), 
        variable, 
        span,
    );
}

#[derive(Debug, Clone, PartialEq)]
enum IfKind {
    If,
    Else,
    ElseIf,

    End
}

impl IfKind {
    pub fn from_stream(stream: &TokenStream, is_statment: bool) -> Result<Self> {
        
        let peek_i = if stream.current_is("\n") {
            1
        }
        else {            
            0
        };

        match stream.peek_multiple(peek_i)
            .map(|el| el.text.as_str())
            .ok_or(err_out_of_bounds(stream))?
        {
            
            val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::If) => {
                Ok(Self::If)
            }
            val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Else) => {
                if stream.peek_multiple_is(peek_i+1, "if") {
                    Ok(Self::ElseIf)
                }
                else {
                    Ok(Self::Else)
                }
            },
            _ => {
                if is_statment && STATMENT_END_TOKENS.iter().any(|token| *token == stream.current_text()) {
                    Ok(Self::End)
                }
                else {
                    Err(new_soul_error(
                        SoulErrorKind::InvalidInContext, 
                        stream.current_span_some(), 
                        format!("token: '{}' not allowed after 'if' (try adding 'else' first)", stream.current_text())
                    ))
                }
            }
        }
    }
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span_some(), "unexpected end while parsing expression")
}


























