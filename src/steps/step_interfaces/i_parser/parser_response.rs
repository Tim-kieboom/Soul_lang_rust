use crate::errors::soul_error::Result;
use crate::steps::step_interfaces::{i_parser::{abstract_syntax_tree::abstract_syntax_tree::AbstractSyntacTree, scope::ScopeBuilder}, i_tokenizer::TokenStream};
 
pub trait FromTokenStream<T> {
    fn try_from_stream(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Option<Result<T>>;
    fn from_stream(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Result<T>;
}

pub struct ParserResponse {
    pub tree: AbstractSyntacTree,
    pub scopes: ScopeBuilder,
}
























