use crate::steps::step_interfaces::i_tokenizer::Token;
use crate::steps::parser::expression::literal::{add_literal, try_get_literal};
use crate::steps::parser::expression::symbool::{Bracket, Operator, Symbool, SymboolKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::pretty_format::ToString;
use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind, SoulSpan};
use crate::steps::step_interfaces::{i_parser::scope_builder::ScopeBuilder, i_tokenizer::TokenStream};
use crate::steps::parser::expression::merge_expression::{get_binary_expression, get_unary_expression, merge_expressions};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{BinaryOperatorKind, Expression, ExpressionKind, Ternary, UnaryOperatorKind};


pub struct ExprOptions {
    pub is_assign_var: bool,
    pub open_bracket_stack: i64,
}

impl Default for ExprOptions {
    fn default() -> Self {
        Self { 
            is_assign_var: false,
            open_bracket_stack: 0,
        }
    }
}

pub fn get_expression(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    end_tokens: &[&str],
) -> Result<Expression> {
    inner_get_expression(stream, scopes, end_tokens, ExprOptions::default())
}

pub fn get_expression_options(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    end_tokens: &[&str],
    options: ExprOptions,
) -> Result<Expression> {
    inner_get_expression(stream, scopes, end_tokens, options)
}

fn inner_get_expression(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    end_tokens: &[&str],
    mut options: ExprOptions,
) -> Result<Expression> {
    let begin_i = stream.current_index();
    let mut stacks = ExpressionStacks::new();

    let result = convert_expression(stream, scopes, &mut stacks, end_tokens, &mut options);
    if result.is_err() {
        stream.go_to_index(begin_i);
        return Err(result.unwrap_err());
    }

    while let Some(operator) = stacks.symbools.pop() {

        let span = operator.span;
        let expression = match operator.node {
            SymboolKind::BinaryOperator(binary_operator) => get_binary_expression(&mut stacks, binary_operator, span)?,
            SymboolKind::UnaryOperator(unary_operator) => get_unary_expression(&mut stacks, unary_operator, span)?,
            SymboolKind::Bracket(_) => stacks.expressions.pop()
                .expect("Internal Error found Symbool::Bracket in convert expression while expressionStack is empty"),
        };
    
        stacks.expressions.push(expression);
    }

    if stacks.expressions.is_empty() {
        debug_assert!(
            stacks.symbools.is_empty(), 
            "Internal error: in get_expression() stacks.node_stack.is_empty() but node_stack is not"
        );

        return Ok(Expression::new(ExpressionKind::Empty, stream[begin_i].span));
    }

    if stacks.expressions.len() > 1 {

        let right = stacks.expressions.pop().unwrap().node;
        let left = stacks.expressions.pop().unwrap().node;

        return Err(new_soul_error(
            SoulErrorKind::InvalidInContext, 
            stream[begin_i].span, 
            format!("expression: '{}' with '{}' is invalid (missing operator)", left.to_string(), right.to_string())
        ))
    }

    Ok(stacks.expressions.pop().unwrap())
}

fn convert_expression(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks, 
    end_tokens: &[&str],
    options: &mut ExprOptions,
) -> Result<()> {
    
    stream.next_multiple(-1);

    loop {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if is_end_token(stream.current(), end_tokens, options) {
            break
        }

        let literal_begin = stream.current_index();
        let possible_literal = try_get_literal(stream, scopes, stacks, options)?;

        if let Some(literal) = possible_literal {
            let literal_span = stream[literal_begin].span.combine(&stream.current_span());
            
            add_literal(literal, scopes, stacks, literal_span);
            end_loop(stream, scopes, stacks)?;
            continue;
        }
        else {

        }
        
        if let Some(operator) = get_operator(stream, stacks) {
            try_add_operator(stacks, operator, stream.current_span())?;
        }
        else {

            return Err(new_soul_error(
                SoulErrorKind::UnexpectedToken,
                stream.current_span(), 
                format!("token: '{}' is not valid expression", stream.current_text())
            ));
        }
    }

    Ok(())
}




fn try_add_operator(
    stacks: &mut ExpressionStacks,
    mut operator: Operator,
    span: SoulSpan
) -> Result<()> {

    merge_expressions(stacks, operator.get_precedence())?;

    if operator == Operator::Binary(BinaryOperatorKind::Sub) && 
       is_minus_negative_unary(stacks) 
    {
        operator = Operator::Unary(UnaryOperatorKind::Neg)
    }

    stacks.symbools.push(operator.to_symbool(span));
    Ok(())
}


fn get_operator(stream: &TokenStream, stacks: &ExpressionStacks) -> Option<Operator> {
    let mut operator = Operator::from_str(stream.current_text());
    if unary_is_before_expression(&operator, stacks) {
        return operator
    }

    match &mut operator {
        Some(Operator::Unary(UnaryOperatorKind::Increment{before_var})) => *before_var = false,
        Some(Operator::Unary(UnaryOperatorKind::Decrement{before_var})) => *before_var = false,
        Some(_) => (),
        None => return None,
    }

    operator
}




fn end_loop(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks, 
) -> Result<()> { 
    
    if stream.peek_is("[") {
        todo!("add index")
    }

    while stream.peek_is(".") {
        todo!("add_field_or_methode")
    }

    if stream.peek_is("?") {
        add_ternary(stream, scopes, stacks)?;
    }

    //should be ref

    Ok(())
}

fn add_ternary(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks,
) -> Result<()> {
    if stream.next_multiple(2).is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream));
    }

    let condition = if stacks.expressions.len() == 1 {
        Box::new(stacks.expressions.pop().unwrap())
    }
    else {
        let last_symbool = stacks.symbools
            .last()
            .ok_or(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), "missing operator"))?; 

        merge_expressions(stacks, last_symbool.node.get_precedence())?;
        Box::new(stacks.expressions.pop().unwrap())
    };

    let if_branch = Box::new(get_expression(stream, scopes, &["\n", ":"])?);
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if stream.next_if(":").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let else_branch = Box::new(get_expression(stream, scopes, &[";", "\n", "}"])?);
    stream.next_multiple(-1);

    let span = condition.span.combine(&else_branch.span);
    let ternary = Ternary{condition, else_branch, if_branch};
    stacks.expressions.push(Expression::new(ExpressionKind::Ternary(ternary), span));
    Ok(())
}

pub fn traverse_brackets(
    stream: &mut TokenStream, 
    stacks: &mut ExpressionStacks, 
    options: &mut ExprOptions,
) -> bool {
    let token = stream.current_text();
    if token == "(" {
        stacks.symbools.push(Symbool::new(SymboolKind::Bracket(Bracket::RoundOpen), stream.current_span()));
        stream.next();
        
        options.open_bracket_stack += 1;
    } 
    else if token == ")" {
        stacks.symbools.push(Symbool::new(SymboolKind::Bracket(Bracket::RoundClose), stream.current_span()));
        stream.next();

        options.open_bracket_stack -= 1;
        if options.open_bracket_stack >= 0 {
            return true;
        }
    }

    false
}

fn is_minus_negative_unary(stacks: &ExpressionStacks) -> bool {
    stacks.expressions.is_empty() || !stacks.symbools.is_empty()
}

fn unary_is_before_expression(operator: &Option<Operator>, stacks: &ExpressionStacks) -> bool {
    operator.is_none() || stacks.expressions.is_empty() || !has_no_operators(stacks) 
}

fn has_no_operators(stacks: &ExpressionStacks) -> bool {
    stacks.symbools.is_empty() || stacks.symbools.iter().all(|sy| matches!(sy.node, SymboolKind::Bracket(..)))
}

fn is_end_token(token: &Token, end_tokens: &[&str], options: &ExprOptions) -> bool {
    end_tokens.iter().any(|str| str == &token.text) && is_valid_end_token(token, options)

}
fn is_valid_end_token(token: &Token, options: &ExprOptions) -> bool {
    token.text != ")" || (token.text == ")" && options.open_bracket_stack == 0)
}

#[derive(Default)]
pub struct ExpressionStacks {
    pub expressions: Vec<Expression>,
    pub symbools: Vec<Symbool>,
}

impl ExpressionStacks {
    pub fn new() -> Self {
        Self{..Default::default()}
    }
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while parsing exprestion")
}







































