use crate::steps::step_interfaces::{i_parser::abstract_syntax_tree::abstract_syntax_tree::AbstractSyntacTree, i_sementic::{scope_vistitor::ScopeVisitor, soul_fault::SoulFault}};

pub struct SementicResponse {
    pub tree: AbstractSyntacTree,
    pub scope: ScopeVisitor,
    pub faults: Vec<SoulFault>,
}







