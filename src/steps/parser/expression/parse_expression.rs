use crate::steps::step_interfaces::i_tokenizer::Token;
use crate::steps::parser::expression::symbool::{Symbool, SymboolKind};
use crate::steps::step_interfaces::i_parser::scope_builder::ProgramMemmory;
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::literal::Literal;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::pretty_format::ToString;
use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind, SoulSpan};
use crate::steps::step_interfaces::{i_parser::scope_builder::ScopeBuilder, i_tokenizer::TokenStream};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{Binary, BinaryOperator, BinaryOperatorKind, Expression, ExpressionKind, Ternary, Unary, UnaryOperator, UnaryOperatorKind};

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

        if let Some(literal) = Literal::try_from_stream(stream, scopes)? {
            let literal_span = stream[literal_begin].span.combine(&stream.current_span());
            
            add_literal(literal, scopes, stacks, literal_span);
            end_loop(stream, scopes, stacks)?;
            continue;
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


fn add_literal(
    literal: Literal, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks,
    span: SoulSpan,
) {
    let expression = match &literal {
        Literal::Tuple{..} |
        Literal::Array{..} |
        Literal::NamedTuple{..} => {
            let literal_type = literal.get_literal_type();
            let id = scopes.global_literals.insert(literal);
            let name = ProgramMemmory::to_program_memory_name(&id);
            ExpressionKind::Literal(Literal::ProgramMemmory(name, literal_type))
        },
        _ => ExpressionKind::Literal(literal),
    };

    stacks.expressions.push(Expression::new(expression, span));
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

fn merge_expressions(stacks: &mut ExpressionStacks, current_precedence: u8) -> Result<()> {
    
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

fn get_binary_expression(
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

fn get_unary_expression(
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

fn is_end_token(token: &Token, end_tokens: &[&str], options: &ExprOptions) -> bool {
    end_tokens.iter().any(|str| str == &token.text) && is_valid_end_token(token, options)

}
fn is_valid_end_token(token: &Token, options: &ExprOptions) -> bool {
    token.text != ")" || (token.text == ")" && options.open_bracket_stack == 0)
}

#[derive(Default)]
struct ExpressionStacks {
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







































