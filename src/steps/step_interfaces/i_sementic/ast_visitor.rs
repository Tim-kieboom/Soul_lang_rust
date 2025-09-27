use crate::{errors::soul_error::SoulError, steps::step_interfaces::{i_parser::abstract_syntax_tree::abstract_syntax_tree::AbstractSyntacTree, i_sementic::{scope_vistitor::ScopeVisitor, soul_fault::{SoulFault, SoulFaultKind}}}};

pub trait AstAnalyser {
    fn analyse_ast(&mut self, tree: &mut AbstractSyntacTree);
}

/*put macros before here before other analysers*/

pub struct NameResolutionAnalyser {
    scope: ScopeVisitor,
    faults: Vec<SoulFault>,
    has_error: bool,
}

pub struct ExternalHeaderAnalyser {
    scope: ScopeVisitor,
    faults: Vec<SoulFault>,
    has_error: bool,
}

pub struct TraitAnalyser {
    scope: ScopeVisitor,
    faults: Vec<SoulFault>,
    has_error: bool,
}

pub struct TypeResolutionAnalyser {
    scope: ScopeVisitor,
    faults: Vec<SoulFault>,
    has_error: bool,
}

pub struct TypeCheckingAnalyser {
    scope: ScopeVisitor,
    faults: Vec<SoulFault>,
    has_error: bool,
}

pub struct ControlFlowAnalyser {
    scope: ScopeVisitor,
    faults: Vec<SoulFault>,
    has_error: bool,
}

pub struct BorrowChecker {
    scope: ScopeVisitor,
    faults: Vec<SoulFault>,
    has_error: bool,
}

pub struct Optimizer {
    scope: ScopeVisitor,
    faults: Vec<SoulFault>,
    has_error: bool,
}

impl NameResolutionAnalyser {
    pub fn new(scope: ScopeVisitor, should_reset: bool) -> Self {
        Self::inner_new(scope, vec![], false, should_reset)
    }
}

impl ExternalHeaderAnalyser {
    pub fn new(analyser: NameResolutionAnalyser, should_reset: bool) -> Self {
        let (scope, faults, has_error) = analyser.consume_to_tuple();
        Self::inner_new(scope, faults, has_error, should_reset)   
    }
}

impl TypeResolutionAnalyser {
    pub fn new(analyser: ExternalHeaderAnalyser, should_reset: bool) -> Self {
        let (scope, faults, has_error) = analyser.consume_to_tuple();
        Self::inner_new(scope, faults, has_error, should_reset)   
    }
}

impl TypeCheckingAnalyser {
    pub fn new(analyser: TypeResolutionAnalyser, should_reset: bool) -> Self {
        let (scope, faults, has_error) = analyser.consume_to_tuple();
        Self::inner_new(scope, faults, has_error, should_reset)   
    }
}

impl ControlFlowAnalyser {
    pub fn new(analyser: TypeCheckingAnalyser, should_reset: bool) -> Self {
        let (scope, faults, has_error) = analyser.consume_to_tuple();
        Self::inner_new(scope, faults, has_error, should_reset)   
    }
}

impl TraitAnalyser {
    pub fn new(analyser: TraitAnalyser, should_reset: bool) -> Self {
        let (scope, faults, has_error) = analyser.consume_to_tuple();
        Self::inner_new(scope, faults, has_error, should_reset)   
    }
}

impl BorrowChecker {
    pub fn new(analyser: TraitAnalyser, should_reset: bool) -> Self {
        let (scope, faults, has_error) = analyser.consume_to_tuple();
        Self::inner_new(scope, faults, has_error, should_reset)   
    }
}

impl Optimizer {
    pub fn new(analyser: BorrowChecker, should_reset: bool) -> Self {
        let (scope, faults, has_error) = analyser.consume_to_tuple();
        Self::inner_new(scope, faults, has_error, should_reset)   
    }
}

macro_rules! impl_default_methods {
    ( $($ty:ty),+) => {
        $(
            impl $ty {
                fn inner_new(mut scope: ScopeVisitor, faults: Vec<SoulFault>, has_error: bool, should_reset: bool) -> Self {
                    if should_reset {
                        scope.reset();
                    }

                    Self{scope, faults, has_error}
                }

                pub fn add_fault(&mut self, fault: SoulFault) {
                    if let SoulFaultKind::Error = fault.kind {
                        self.has_error = true;
                    }
                    self.faults.push(fault)
                }
                pub fn add_error(&mut self, msg: SoulError) { 
                    self.has_error = true;
                    self.faults.push(SoulFault::new_error(msg)); 
                }
                pub fn add_warning(&mut self, msg: SoulError) { 
                    self.faults.push(SoulFault::new_warning(msg)); 
                }
                pub fn add_note(&mut self, msg: SoulError) { 
                    self.faults.push(SoulFault::new_note(msg)); 
                }

                pub fn has_error(&self) -> bool {self.has_error}
                pub fn get_scope(&self) -> &ScopeVisitor { &self.scope }
                pub fn get_scope_mut(&mut self) -> &mut ScopeVisitor { &mut self.scope }
                pub fn get_faults(&self) -> &Vec<SoulFault> { &self.faults }
                pub fn get_faults_mut(&mut self) -> &mut Vec<SoulFault> { &mut self.faults }
                pub fn consume_to_tuple(self) -> (ScopeVisitor, Vec<SoulFault>, bool) { (self.scope, self.faults, self.has_error) }
            }
        )+
    };
}

impl_default_methods!(
    NameResolutionAnalyser,
    ExternalHeaderAnalyser,
    TypeResolutionAnalyser,
    TypeCheckingAnalyser,
    ControlFlowAnalyser,
    TraitAnalyser,
    BorrowChecker,
    Optimizer
);

