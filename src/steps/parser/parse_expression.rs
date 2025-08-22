use crate::errors::soul_error::Result;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Expression;
use crate::steps::step_interfaces::{i_parser::scope_builder::ScopeBuilder, i_tokenizer::TokenStream};

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

    todo!()
}

fn convert_expression(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks, 
    end_tokens: &[&str],
    options: &mut ExprOptions,
) -> Result<()> {
    
    stream.next_multiple(-1);

    while stream.next().is_some() {

        

    }

    todo!()
}

#[derive(Default)]
pub struct ExpressionStacks {
    pub expressions: Vec<Expression>,
}

impl ExpressionStacks {
    pub fn new() -> Self {
        Self{..Default::default()}
    }
}








