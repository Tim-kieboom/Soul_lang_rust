use crate::errors::soul_error::Result;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{BinOp, UnaryExpr, UnaryOp};
use crate::steps::step_interfaces::i_parser::scope::ScopeBuilder;
use crate::{errors::soul_error::{new_soul_error, SoulErrorKind, SoulSpan}, steps::{parser::get_expressions::{parse_expression::ExpressionStacks, symbool::{SymboolKind, ROUND_BRACKET_CLOSED, ROUND_BRACKET_OPEN}}, step_interfaces::{i_parser::abstract_syntax_tree::expression::{BinaryExpr, Expression}, i_tokenizer::TokenStream}}};

pub fn convert_bracket_expression(stream: &mut TokenStream, scopes: &ScopeBuilder, stacks: &mut ExpressionStacks) -> Result<()> {
    if stacks.node_stack.len() == 1 {
        let first = stacks.symbool_stack.pop().map(|sy| sy.node);
        let mut second = stacks.symbool_stack.pop();

        if first != Some(ROUND_BRACKET_CLOSED) {
            return Err(new_soul_error(SoulErrorKind::InternalError, stream.current_span(), "while doing convert_bracket_expression first symbool is not ')'"));
        }
        else if second.is_none() {
            return Err(new_soul_error(SoulErrorKind::InternalError, stream.current_span(), "while doing convert_bracket_expression second symbool is not None"));
        }

        match &second.as_ref().unwrap().node {
            SymboolKind::BinOp(..) => return Err(new_soul_error(SoulErrorKind::InternalError, stream.current_span(), "while doing convert_bracket_expression second symbool is binary operator")),
            SymboolKind::UnaryOp(unary) => {
                let expr = get_unary_expression(&mut stacks.node_stack, UnaryOp::new(unary.clone(), second.as_ref().unwrap().span), second.as_ref().unwrap().span)?;
                stacks.node_stack.push(expr);
                second = stacks.symbool_stack.pop();
            },
            SymboolKind::Parenthesis(..) =>(),
        }

        if !second.is_some_and(|sy| sy.node == ROUND_BRACKET_OPEN) {
            return Err(new_soul_error(SoulErrorKind::InternalError, stream.current_span(), "while doing convert_bracket_expression second symbool is not None"));
        }

        return Ok(());
    }

    if !stacks.symbool_stack.pop().is_some_and(|symbool| symbool.node == ROUND_BRACKET_CLOSED) {
        return Err(new_soul_error(
            SoulErrorKind::UnmatchedParenthesis, 
            stream.peek_multiple(-1).unwrap_or(stream.current()).span, 
            "in 'getBracketBinairyExpression()': symoboolStack top is not ')'"
        ));
    }

    while let Some(symbool) = stacks.symbool_stack.last() {
        if symbool.node == ROUND_BRACKET_OPEN {
            break;
        }

        if let SymboolKind::BinOp(bin_op) = &symbool.node {
            let expr = get_binary_expression(&mut stacks.node_stack, scopes, BinOp::new(bin_op.clone(), symbool.span), symbool.span)?;
                
            stacks.node_stack.push(expr);
        }
        else if let SymboolKind::UnaryOp(un_op) = &symbool.node {
            let expr = get_unary_expression(&mut stacks.node_stack, UnaryOp::new(un_op.clone(), symbool.span), symbool.span)?;
            
            stacks.node_stack.push(expr);
        }

        stacks.symbool_stack.pop();
    }

    Ok(())
}

pub fn get_binary_expression(
    node_stack: &mut Vec<Expression>,
    scopes: &ScopeBuilder,
    bin_op: BinOp,
    span: SoulSpan,
) -> Result<Expression> {
    let right = node_stack.pop()
        .map(|expr| Box::new(expr))
        .ok_or(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            span, 
            format!("found binary operator '{}' but no expression", bin_op.node.to_str())
        ))?;

    let left = node_stack.pop()
        .map(|expr| Box::new(expr))
        .ok_or(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            span, 
            format!("missing right expression in binary expression (so '{} {} <missing>')", right.node.to_string(&scopes.ref_pool, 0), bin_op.node.to_str())
        ))?;

    let new_span = right.span.combine(&left.span);
    Ok(BinaryExpr{left, operator: bin_op, right}.consume_to_expression(new_span))
}

pub fn get_unary_expression(
    node_stack: &mut Vec<Expression>,
    unary_op: UnaryOp,
    span: SoulSpan, 
) -> Result<Expression> {
    let expr = node_stack.pop()
        .ok_or(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            span, 
            format!("found unary operator '{}' but no expression", unary_op.node.to_str())
        ))?;
    
    let new_span = expr.span.combine(&unary_op.span);
    Ok(UnaryExpr{operator: unary_op, expression: Box::new(expr)}.consume_to_expression(new_span))
}











































