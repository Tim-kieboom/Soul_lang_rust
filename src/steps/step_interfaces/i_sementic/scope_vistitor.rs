use crate::steps::step_interfaces::i_parser::{abstract_syntax_tree::{spanned::Spanned}, header::{ExternalHeaders}, scope_builder::{InnerScope, ProgramMemmory, ScopeBuilder, ScopeId, ScopeKind}};

pub type Scope = InnerScope<Vec<Spanned<ScopeKind>>>;

pub struct ScopeVisitor {
    scopes: Vec<Scope>,
    current: ScopeId,
    pub global_literals: ProgramMemmory,
    pub project_name: String,
    pub external_headers: ExternalHeaders,
}

impl ScopeVisitor {

    pub const GLOBAL_SCOPE_INDEX: ScopeId = ScopeId(0);

    pub fn new(scope: ScopeBuilder, external_headers: ExternalHeaders) -> Self {
        let (scopes, current, global_literals, project_name) = scope.__consume_to_tuple();
        Self {
            scopes,
            current,
            project_name,
            global_literals,
            external_headers,
        }
    }

    pub fn reset(&mut self) {
        self.current = Self::GLOBAL_SCOPE_INDEX;
        for scope in &mut self.scopes {
            scope.current_child = ScopeId(0);
        }
    }

    pub fn set_current(&mut self, id: ScopeId) -> Option<&Scope> {
        self.current = id;
        self.scopes.get(self.current.0)
    }

    pub fn is_in_global(&self) -> bool {
        self.current == Self::GLOBAL_SCOPE_INDEX
    } 

    pub fn flat_lookup(&self, name: &str) -> Option<&Vec<Spanned<ScopeKind>>> {
        let scope = &self.scopes[self.current.0];

        if let Some(kinds) = scope.get(name) {
            return Some(kinds);
        }

        None
    }
        
    pub fn flat_lookup_mut(&mut self, name: &str) -> Option<&mut Vec<Spanned<ScopeKind>>> {
        let scope = &mut self.scopes[self.current.0];

        if let Some(kinds) = scope.get_mut(name) {
            return Some(kinds);
        }

        None
    }

    pub fn lookup(&self, name: &str) -> Option<&Vec<Spanned<ScopeKind>>> {
        let mut current_index = Some(self.current);

        while let Some(index) = current_index {
            let scope = &self.scopes[index.0];

            if let Some(kinds) = scope.get(name) {
                return Some(kinds);
            }

            current_index = scope.parent_index;
        }

        None
    }

    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut Vec<Spanned<ScopeKind>>> {
        let mut current_index = Some(self.current);

        while let Some(index) = current_index {
            let has_kinds = self.scopes[index.0].get(name).is_some();

            if has_kinds {
                return self.scopes[index.0].get_mut(name)
            }

            current_index = self.scopes[index.0].parent_index;
        }

        None
    }

    pub fn current_id(&self) -> ScopeId {
        self.current
    }

    pub fn current_scope(&self) -> &InnerScope<Vec<Spanned<ScopeKind>>> {
        &self.scopes[self.current.0]
    }

    pub fn current_scope_mut(&mut self) -> &mut InnerScope<Vec<Spanned<ScopeKind>>> {
        &mut self.scopes[self.current.0]
    }

    pub fn get_scopes(&self) -> &Vec<Scope> {
        &self.scopes
    }

    pub fn get_scopes_mut(&mut self) -> &mut Vec<Scope> {
        &mut self.scopes
    }

}
