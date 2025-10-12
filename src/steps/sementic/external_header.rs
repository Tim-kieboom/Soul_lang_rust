use crate::{steps::step_interfaces::{i_parser::{abstract_syntax_tree::{abstract_syntax_tree::AbstractSyntacTree}}, i_sementic::{ast_visitor::{AstAnalyser, ExternalHeaderAnalyser}}}};

impl AstAnalyser for ExternalHeaderAnalyser {
    fn analyse_ast(&mut self, _tree: &mut AbstractSyntacTree) {}
}