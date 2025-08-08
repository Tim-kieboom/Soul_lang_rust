use crate::steps::step_interfaces::{i_parser::abstract_syntax_tree::abstract_syntax_tree::AbstractSyntacTree, i_sementic::{fault::SoulFault, sementic_scope::ScopeVisitor}};

pub struct SementicAnalyserResult {
    pub tree: AbstractSyntacTree,
    pub scope: ScopeVisitor,
    pub faults: Vec<SoulFault>,
}















