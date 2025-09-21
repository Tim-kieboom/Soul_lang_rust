use crate::steps::step_interfaces::{i_parser::abstract_syntax_tree::abstract_syntax_tree::AbstractSyntacTree, i_sementic::{scope_vistitor::ScopeVisitor, soul_fault::SoulFault}};

pub struct SementicResponse {
    pub tree: AbstractSyntacTree,
    pub scopes: ScopeVisitor,
    pub faults: Vec<SoulFault>,
    pub has_error: bool,
}







