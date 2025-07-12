use crate::steps::step_interfaces::i_parser::{abstract_syntax_tree::abstract_syntax_tree::AbstractSyntacTree, scope::ScopeBuilder};

pub struct ParserResponse {
    pub tree: AbstractSyntacTree,
    pub scopes: ScopeBuilder,
}
























