use crate::prelude::*;
use my_macros::{CloneWithPool};
use serde::{Deserialize, Serialize};
use crate::errors::soul_error::Result;
use crate::steps::step_interfaces::{i_parser::{abstract_syntax_tree::abstract_syntax_tree::AbstractSyntacTree, scope::ScopeBuilder}, i_tokenizer::TokenStream};
 
pub trait FromTokenStream<T> {
    fn try_from_stream(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Option<Result<T>>;
    fn from_stream(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<T>;
}

#[derive(Debug, Clone, CloneWithPool, Serialize, Deserialize)]
pub struct ParserResponse {
    pub tree: AbstractSyntacTree,
    pub scopes: ScopeBuilder,
}
























