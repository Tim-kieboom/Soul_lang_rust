use crate::steps::step_interfaces::i_parser::{abstract_syntax_tree::spanned::Spanned, scope_builder::{InnerScope, ProgramMemmory, ScopeBuilder, ScopeKind}};

type Scope = InnerScope<Vec<Spanned<ScopeKind>>>;

pub struct ScopeVisitor {
    scopes: Vec<Scope>,
    current: usize,
    pub global_literals: ProgramMemmory,
    pub project_name: String,
}

impl ScopeVisitor {

    const GLOBAL_SCOPE_INDEX: usize = 0;

    pub fn new(scope: ScopeBuilder) -> Self {
        let (scopes, current, global_literals, project_name) = scope.__consume_to_tuple();
        Self {
            scopes,
            current,
            project_name,
            global_literals,
        }
    }

    pub fn reset(&mut self) {
        self.current = Self::GLOBAL_SCOPE_INDEX
    }

    pub fn next_child(&mut self) -> bool {
        if let Some(&first_child) = self.scopes[self.current].children.first() {
            self.current = first_child;
            true
        }
        else {
            false
        }
    }

    pub fn next_sibling(&mut self) -> bool {
        let this = &self.scopes[self.current];

        let parent = match this.parent_index {
            Some(val) => val,
            None => return false,
        };

        let children = &self.scopes[parent].children;
        if children.is_empty() {
            return false
        }

        let this_index = children.iter().position(|&i| i == this.self_index).expect("this_index in parent.children not found");
        if this_index == children.len()-1 {
            return false;
        }

        self.current = children[this_index+1];
        true
    }

    pub fn to_parent(&mut self) -> bool {
        if let Some(parent) = self.scopes[self.current].parent_index {
            self.current = parent;
            true
        } 
        else {
            false
        }
    }

    pub fn is_in_global(&self) -> bool {
        self.current == Self::GLOBAL_SCOPE_INDEX
    } 

    pub fn flat_lookup(&self, name: &str) -> Option<&Vec<Spanned<ScopeKind>>> {
        let scope = &self.scopes[self.current];

        if let Some(kinds) = scope.get(name) {
            return Some(kinds);
        }

        None
    }
        
    pub fn flat_lookup_mut(&mut self, name: &str) -> Option<&mut Vec<Spanned<ScopeKind>>> {
        let scope = &mut self.scopes[self.current];

        if let Some(kinds) = scope.get_mut(name) {
            return Some(kinds);
        }

        None
    }

    pub fn lookup(&self, name: &str) -> Option<&Vec<Spanned<ScopeKind>>> {
        let mut current_index = Some(self.current);

        while let Some(index) = current_index {
            let scope = &self.scopes[index];

            if let Some(kinds) = scope.get(name) {
                return Some(kinds);
            }

            current_index = scope.parent_index;
        }

        None
    }

    pub fn current_scope(&self) -> &InnerScope<Vec<Spanned<ScopeKind>>> {
        &self.scopes[self.current]
    }

    pub fn current_scope_mut(&mut self) -> &mut InnerScope<Vec<Spanned<ScopeKind>>> {
        &mut self.scopes[self.current]
    }

    pub fn get_scopes(&self) -> &Vec<Scope> {
        &self.scopes
    }
}
