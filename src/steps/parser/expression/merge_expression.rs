use crate::errors::soul_error::Result;
use crate::steps::parser::expression::symbool::Bracket;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::pretty_format::ToString;
use crate::steps::step_interfaces::i_tokenizer::TokenStream;
use crate::{errors::soul_error::{new_soul_error, SoulErrorKind, SoulSpan}, steps::{parser::expression::{parse_expression::ExpressionStacks, symbool::SymboolKind}, step_interfaces::i_parser::abstract_syntax_tree::expression::{Binary, BinaryOperator, BinaryOperatorKind, Expression, ExpressionKind, Unary, UnaryOperator, UnaryOperatorKind}}};

const BRACKET_OPEN: SymboolKind = SymboolKind::Bracket(Bracket::RoundOpen);
const BRACKET_CLOSE: SymboolKind = SymboolKind::Bracket(Bracket::RoundClose);

pub fn merge_expressions(stacks: &mut ExpressionStacks, current_precedence: u8) -> Result<()> {
    
    fn last_precedence(stacks: &mut ExpressionStacks) -> u8 {
        stacks.symbools.last().unwrap().node.get_precedence()
    }

    fn is_last_precedence_greater(stacks: &mut ExpressionStacks, current_precedence: u8) -> bool {
        !stacks.symbools.is_empty() && last_precedence(stacks) >= current_precedence  
    }

    while is_last_precedence_greater(stacks, current_precedence) { 
        let operator = stacks.symbools.pop().unwrap();
        let expression = match operator.node {
            SymboolKind::UnaryOperator(unary_operator) => get_unary_expression(stacks, unary_operator, operator.span)?,
            SymboolKind::BinaryOperator(binary_operator) => get_binary_expression(stacks, binary_operator, operator.span)?,
            SymboolKind::Bracket(..) => panic!("Internal error this should not be possible, precedence should be 0 and all valid ops > 0"),
        };

        stacks.expressions.push(expression);
    }

    Ok(())
}

pub fn convert_bracket_expression(
    stream: &mut TokenStream, 
    stacks: &mut ExpressionStacks,
) -> Result<()> {

    if stacks.expressions.len() == 1 {
        return convert_single(stream, stacks);
    }    

    if !stacks.symbools.pop().is_some_and(|symbool| symbool.node == BRACKET_CLOSE) {
        return Err(new_soul_error(
            SoulErrorKind::UnmatchedParenthesis, 
            stream.peek_multiple(-1).unwrap_or(stream.current()).span, 
            "in 'getBracketBinairyExpression()': symoboolStack top is not ')'"
        ));
    }

    while let Some(symbool) = stacks.symbools.last() {
        if symbool.node == BRACKET_OPEN {
            break;
        }

        if let SymboolKind::BinaryOperator(bin_op) = &symbool.node {
            let expr = get_binary_expression(stacks, bin_op.clone(), symbool.span)?;
                
            stacks.expressions.push(expr);
        }
        else if let SymboolKind::UnaryOperator(un_op) = &symbool.node {
            let expr = get_unary_expression(stacks, un_op.clone(), symbool.span)?;
            
            stacks.expressions.push(expr);
        }

        stacks.symbools.pop();
    }

    Ok(())
}

fn convert_single(
    stream: &mut TokenStream, 
    stacks: &mut ExpressionStacks,
) -> Result<()> {
    let first = stacks.symbools.pop()
        .map(|el| el.node);

    let mut second = stacks.symbools.pop();

    if first != Some(BRACKET_CLOSE) {
        return Err(new_soul_error(SoulErrorKind::InternalError, stream.current_span(), "while doing convert_bracket_expression first symbool is not ')'"));
    }
    
    if second.is_none() {
        return Err(new_soul_error(SoulErrorKind::InternalError, stream.current_span(), "while doing convert_bracket_expression second symbool is not None"));
    }

    match &second.as_ref().unwrap().node {
        SymboolKind::BinaryOperator(_) => return Err(new_soul_error(SoulErrorKind::InternalError, stream.current_span(), "while doing convert_bracket_expression second symbool is binary operator")),
        SymboolKind::UnaryOperator(unary_operator) => {
            let expr = get_unary_expression(stacks, unary_operator.clone(), second.as_ref().unwrap().span)?;
            stacks.expressions.push(expr);
            second = stacks.symbools.pop();
        },
        SymboolKind::Bracket(_) => (),
    }

    if !second.is_some_and(|symbool| symbool.node == BRACKET_OPEN) {
        return Err(new_soul_error(
            SoulErrorKind::InternalError, 
            stream.current_span(), 
            "while doing convert_bracket_expression second symbool is not None",
        ));
    }

    Ok(())
}

pub fn get_binary_expression(
    stacks: &mut ExpressionStacks,
    operator: BinaryOperatorKind,
    span: SoulSpan,
) -> Result<Expression> {
    let right = stacks.expressions.pop()
        .ok_or(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            span, 
            format!("found binary operator '{}' but no expression", operator.to_str()),
        ))?;

    let left = stacks.expressions.pop()
        .ok_or(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            span, 
            format!("missing right expression in binary expression (so '{} {} <missing>')", right.node.to_string(), operator.to_str()),
        ))?;

    let binary_span = right.span.combine(&left.span);
    Ok(Expression::new(
        ExpressionKind::Binary(Binary::new(
            left, 
            BinaryOperator::new(operator, span), 
            right
        )), 
        binary_span,
    ))
}

pub fn get_unary_expression(
    stacks: &mut ExpressionStacks,
    operator: UnaryOperatorKind,
    span: SoulSpan, 
) -> Result<Expression> {
    let expr = stacks.expressions.pop()
        .ok_or(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            span, 
            format!("found unary operator '{}' but no expression", operator.to_str())
        ))?;
    
    let unary_span = expr.span.combine(&span);
    Ok(Expression::new(
        ExpressionKind::Unary(Unary{
            operator: UnaryOperator::new(operator, span), 
            expression: Box::new(expr),
        }), 
        unary_span
    ))
}
















