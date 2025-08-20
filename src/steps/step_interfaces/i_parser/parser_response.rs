use serde::{Deserialize, Serialize};

use crate::errors::soul_error::Result;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::abstract_syntax_tree::AbstractSyntacTree;
use crate::steps::step_interfaces::i_parser::scope_builder::ScopeBuilder;
use crate::steps::step_interfaces::i_tokenizer::TokenStream;


pub trait FromTokenStream<T> {
    fn try_from_stream(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Option<T>>;
    fn from_stream(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<T>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserResponse {
    pub tree: AbstractSyntacTree,
    pub scopes: ScopeBuilder,
}













