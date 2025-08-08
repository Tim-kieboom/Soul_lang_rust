use crate::steps::step_interfaces::{i_parser::{abstract_syntax_tree::{abstract_syntax_tree::{AbstractSyntacTree, GlobalNode}, expression::Expression, staments::statment::{Block, Statment}}}, i_sementic::{fault::SoulFault, sementic_scope::ScopeVisitor}};

trait AstVisitable {
    fn visit_ast(&mut self, node: &mut AbstractSyntacTree);
    
    fn visit_global_node(&mut self, node: &mut GlobalNode);
    fn visit_expression(&mut self, node: &mut Expression);
    fn visit_statment(&mut self, node: &mut Statment);
    fn visit_block(&mut self, node: &mut Block);
}

// pub struct MarcoExpansion { // maybe include NameResolution(aka is var in scope oke)
//     scope: ScopeVisitor,
//     faults: Vec<SoulFault>,
// }

//impl AstVisitable in implement mod
pub struct ExternalHeaderAnalyser {
    scope: ScopeVisitor,
    faults: Vec<SoulFault>,
}

//impl AstVisitable in implement mod
pub struct TypeCollector {
    scope: ScopeVisitor,
    faults: Vec<SoulFault>,
}

//impl AstVisitable in implement mod
pub struct TraitAnalyser {
    scope: ScopeVisitor,
    faults: Vec<SoulFault>,
}

//impl AstVisitable in implement mod
pub struct BorrowChecker {
    scope: ScopeVisitor,
    faults: Vec<SoulFault>,
}

//impl AstVisitable in implement mod
pub struct ConstEvaluator {
    scope: ScopeVisitor,
    faults: Vec<SoulFault>,
}

//impl AstVisitable in implement mod
pub struct ControlFlowAnalyser {
    scope: ScopeVisitor,
    faults: Vec<SoulFault>,
}

//impl AstVisitable in implement mod
pub struct Optimizer {
    scope: ScopeVisitor,
    faults: Vec<SoulFault>,
}

macro_rules! impl_default_methodes {
    ($($struct:ty),+) => {
        $(
            impl $struct {
                pub fn new(mut scope: ScopeVisitor, faults: Vec<SoulFault>, should_reset: bool) -> Self {
                    if should_reset {
                        scope.reset();
                    }

                    Self{scope, faults}
                }

                pub fn get_scope(&self) -> &ScopeVisitor {
                    &self.scope
                }

                pub fn get_scope_mut(&mut self) -> &mut ScopeVisitor {
                    &mut self.scope
                }

                pub fn add_fault(&mut self, fault: SoulFault) {
                    self.faults.push(fault);
                }

                pub fn get_faults(&self) -> &Vec<SoulFault> {
                    &self.faults
                }

                pub fn consume(self) -> (ScopeVisitor, Vec<SoulFault>) {
                    (self.scope, self.faults)
                }
            }
        )+
    };
}

impl_default_methodes!(TypeCollector, ExternalHeaderAnalyser, TraitAnalyser, BorrowChecker, ConstEvaluator, ControlFlowAnalyser, Optimizer);
















