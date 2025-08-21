use crate::errors::soul_error::Result;
use crate::steps::parser::expression::merge_expression::convert_bracket_expression;
use crate::steps::parser::expression::parse_expression::traverse_brackets;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{Expression, ExpressionKind};
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::steps::step_interfaces::i_parser::scope_builder::ProgramMemmory;
use crate::{errors::soul_error::SoulSpan, steps::{parser::expression::parse_expression::{ExprOptions, ExpressionStacks}, step_interfaces::{i_parser::{abstract_syntax_tree::literal::Literal, scope_builder::ScopeBuilder}, i_tokenizer::TokenStream}}};

const CLOSED_A_BRACKET: bool = true;

pub fn try_get_literal(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks, 
    options: &mut ExprOptions,
) -> Result<Option<Literal>> {
    match Literal::try_from_stream(stream, scopes)? {
        Some(literal) => Ok(Some(literal)),
        None => {
            if traverse_brackets(stream, stacks, options) == CLOSED_A_BRACKET {
                convert_bracket_expression(stream, stacks)?;
            } 

            Literal::try_from_stream(stream, scopes)
        },
    }
}

pub fn add_literal(
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















