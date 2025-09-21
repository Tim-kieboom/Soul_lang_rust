use std::path::PathBuf;

use crate::steps::step_interfaces::i_parser::{abstract_syntax_tree::spanned::Spanned, scope_builder::{InnerScope, ProgramMemmory, ScopeBuilder, ScopeKind}};

type Scope = InnerScope<Vec<Spanned<ScopeKind>>>;

pub struct ScopeVisitor {
    scopes: Vec<Scope>,
    current: usize,
    pub global_literals: ProgramMemmory,
    pub project_name: String,
    pub file_path: PathBuf,
}

impl ScopeVisitor {

    const GLOBAL_SCOPE_INDEX: usize = 0;

    pub fn new(scope: ScopeBuilder, file_path: PathBuf) -> Self {
        let (scopes, current, global_literals, project_name) = scope.__consume_to_tuple();
        Self {
            scopes,
            current,
            file_path,
            project_name,
            global_literals,
        }
    }

    pub fn reset(&mut self) {
        self.current = Self::GLOBAL_SCOPE_INDEX;
        for scope in &mut self.scopes {
            scope.current_child = 0;
        }
    }

    pub fn push(&mut self) -> Option<&Scope> {
        let scope = self.scopes.get_mut(self.current)?;
        
        self.current = *scope.children.get(scope.current_child)?;
        scope.current_child += 1;
        self.scopes.get(self.current)
    }

    pub fn pop(&mut self) -> Option<&Scope> {
        self.current = self.scopes.get(self.current)?.parent_index?;
        self.scopes.get(self.current)
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
