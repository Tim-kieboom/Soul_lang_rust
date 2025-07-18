use crate::errors::soul_error::Result;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{BinOp, ExprKind, UnaryExpr, UnaryOp, UnaryOpKind};
use crate::{errors::soul_error::{new_soul_error, SoulErrorKind, SoulSpan}, steps::{parser::get_expressions::{parse_expression::ExpressionStacks, symbool::{SymboolKind, ROUND_BRACKET_CLOSED, ROUND_BRACKET_OPEN}}, step_interfaces::{i_parser::abstract_syntax_tree::expression::{BinOpKind, BinaryExpr, Expression}, i_tokenizer::TokenStream}}};

pub fn get_unary_expression(
    node_stack: &mut Vec<Expression>,
    unary_op: UnaryOpKind,
    span: SoulSpan, 
) -> Result<UnaryExpr> {
    let expr = node_stack.pop()
        .ok_or(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            span, 
            format!("found unary operator '{}' but no expression", unary_op.to_str())
        ))?;
    
    Ok(UnaryExpr{operator: UnaryOp::new(unary_op, span), expression: Box::new(expr)})
}

pub fn convert_bracket_expression(stream: &mut TokenStream, stacks: &mut ExpressionStacks) -> Result<()> {
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
                let expr = get_unary_expression(&mut stacks.node_stack, unary.clone(), second.as_ref().unwrap().span)?;
                stacks.node_stack.push(Expression::new(ExprKind::Unary(expr), second.as_ref().unwrap().span));
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
            let expr = get_binary_expression(&mut stacks.node_stack, bin_op.clone(), symbool.span)?
                .consume_to_expression(symbool.span);

            stacks.node_stack.push(expr);
        }
        else if let SymboolKind::UnaryOp(un_op) = &symbool.node {
            let expr = get_unary_expression(&mut stacks.node_stack, un_op.clone(), symbool.span)?
                .consume_to_expression(symbool.span);
            
            stacks.node_stack.push(expr);
        }

        stacks.symbool_stack.pop();
    }

    Ok(())
}

pub fn get_binary_expression(
    node_stack: &mut Vec<Expression>,
    bin_op: BinOpKind,
    span: SoulSpan,
) -> Result<BinaryExpr> {
    let right = node_stack.pop()
        .map(|expr| Box::new(expr))
        .ok_or(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            span, 
            format!("found binary operator '{}' but no expression", bin_op.to_str())
        ))?;

    let left = node_stack.pop()
        .map(|expr| Box::new(expr))
        .ok_or(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            span, 
            format!("missing right expression in binary expression (so '{} {} <missing>')", right.node.to_string(), bin_op.to_str())
        ))?;

    Ok(BinaryExpr{left, operator: BinOp::new(bin_op, span), right})
}













































