use std::io;

use serde::{Deserialize, Serialize};
use crate::{run_options::run_options::RunOptions, steps::step_interfaces::i_parser::{abstract_syntax_tree::soul_type::type_kind::TypeKind, external_header::ExternalHeader, scope::{InnerScope, ProgramMemmory, ScopeBuilder, ScopeKind, ScopeStack, ScopeVisibility, TypeScope}}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeVisitor {
    scopes: InnerScopeVisitor,
    types: Vec<TypeScope>,
    pub global_literal: ProgramMemmory,
    pub external_header: ExternalHeader,
    pub project_name: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerScopeVisitor {
    pub scopes: Vec<Scope<Vec<ScopeKind>>>,
    pub current: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scope<T> {
    pub scope: InnerScope<T>,
    pub current: usize,
}

impl ScopeVisitor {
    pub fn new(builder: ScopeBuilder, run_options: &RunOptions) -> io::Result<Self> {
        let (scopes, types, global_literal, external_pages, project_name) = builder.__consume_to_tuple();
        Ok(Self{
            scopes: InnerScopeVisitor::new(scopes),
            types,
            global_literal,
            external_header: ExternalHeader::new(external_pages, run_options)?,
            project_name,
        })
    }

    pub fn reset(&mut self) {
        self.scopes.current = InnerScopeVisitor::GLOBAL_SCOPE_INDEX;
    }

    pub fn next_child(&mut self) -> bool {
        self.scopes.next_child()
    }

    pub fn to_parent(&mut self) -> bool {
        self.scopes.to_parent()
    }

    pub fn is_in_global(&self) -> bool {
        self.scopes.is_in_global()
    } 

    pub fn get_scopes(&self) -> &InnerScopeVisitor {
        &self.scopes
    }

    pub fn get_types(&self) -> &Vec<InnerScope<TypeKind>> {
        &self.types
    }

    ///only looks in current scope
    pub fn flat_lookup(&self, name: &str) -> Option<&Vec<ScopeKind>> {
        self.scopes.flat_lookup(name)
    }
    
    ///looks in current scope and parent scopes of ScopeVisibilty is All
    pub fn lookup(&self, name: &str) -> Option<&Vec<ScopeKind>> {
        self.scopes.lookup(name)
    }

    ///looks in current scope and parent scopes of ScopeVisibilty is All
    pub fn lookup_fn<F, T>(&self, name: &str, func: F, other: &T) -> Option<&Vec<ScopeKind>> 
    where 
        F: Fn(&Vec<ScopeKind>, &T) -> bool
    {
        self.scopes.lookup_fn(name, func, other)
    }

    pub fn insert(&mut self, name: String, kind: ScopeKind)  {
        self.scopes.current_mut()
            .symbols
            .entry(name)
            .or_default()
            .push(kind);
    }

    pub fn lookup_type(&self, name: &str) -> Option<&TypeKind> {
        let mut current_index = Some(self.scopes.current);

        while let Some(index) = current_index {
            #[cfg(debug_assertions)]
            if index > self.types.len() -1 {
                break;
            }
            
            let scope = &self.types[index];

            if let Some(kind) = scope.get(name) {
                return Some(kind);
            }

            match scope.visibility_mode {
                ScopeVisibility::All => current_index = scope.parent_index,
                ScopeVisibility::GlobalOnly => {
                    current_index = if index == 0 { None } else { Some(0) };
                }
            }
        }

        None
    }

    pub fn get_global_scope(&self) -> &InnerScope<Vec<ScopeKind>>{
        &self.scopes.scopes[InnerScopeVisitor::GLOBAL_SCOPE_INDEX].scope
    }

    pub fn get_global_types(&self) -> &InnerScope<TypeKind>{
        &self.types[InnerScopeVisitor::GLOBAL_SCOPE_INDEX]
    }
}

impl InnerScopeVisitor {
    pub const GLOBAL_SCOPE_INDEX: usize = 0; 

    pub fn new(stack: ScopeStack) -> Self {
        let ScopeStack{scopes, current:_} = stack;
        Self{
            scopes: scopes.into_iter().map(|scope| Scope{scope, current: 0}).collect(),
            current: Self::GLOBAL_SCOPE_INDEX,
        }
    }

    pub fn next_child(&mut self) -> bool {
        let scope = &mut self.scopes[self.current];
        if let Some(&first_child) = scope.scope.children.get(scope.current) {
            scope.current += 1;
            self.current = first_child;
            true
        } 
        else {
            false
        }
    }

    pub fn to_parent(&mut self) -> bool {
        if let Some(parent) = self.scopes[self.current].scope.parent_index {
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

    pub fn flat_lookup(&self, name: &str) -> Option<&Vec<ScopeKind>> {
        let scope = &self.scopes[self.current];

        if let Some(kinds) = scope.scope.get(name) {
            return Some(kinds);
        }

        None
    }
        
    pub fn flat_lookup_mut(&mut self, name: &str) -> Option<&mut Vec<ScopeKind>> {
        let scope = &mut self.scopes[self.current];

        if let Some(kinds) = scope.scope.get_mut(name) {
            return Some(kinds);
        }

        None
    }

    pub fn lookup_fn<F, T>(&self, name: &str, func: F, other: &T) -> Option<&Vec<ScopeKind>> 
    where 
        F: Fn(&Vec<ScopeKind>, &T) -> bool
    {
        let mut current_index = Some(self.current);

        while let Some(index) = current_index {
            let scope = &self.scopes[index];

            if let Some(kinds) = scope.scope.get(name) {
                
                if func(kinds, other) {
                    return Some(kinds);
                }
            }

            match scope.scope.visibility_mode {
                ScopeVisibility::All => current_index = scope.scope.parent_index,
                ScopeVisibility::GlobalOnly => {
                    current_index = if index == 0 { None } else { Some(0) };
                }
            }
        }

        None
    }

    pub fn lookup(&self, name: &str) -> Option<&Vec<ScopeKind>> {
        let mut current_index = Some(self.current);

        while let Some(index) = current_index {
            let scope = &self.scopes[index];

            if let Some(kinds) = scope.scope.get(name) {
                return Some(kinds);
            }

            match scope.scope.visibility_mode {
                ScopeVisibility::All => current_index = scope.scope.parent_index,
                ScopeVisibility::GlobalOnly => {
                    current_index = if index == 0 { None } else { Some(0) };
                }
            }
        }

        None
    }

    pub fn current(&self) -> &InnerScope<Vec<ScopeKind>> {
        &self.scopes[self.current].scope
    }

    pub fn current_mut(&mut self) -> &mut InnerScope<Vec<ScopeKind>> {
        &mut self.scopes[self.current].scope
    }

    pub fn global_mut(&mut self) -> &mut InnerScope<Vec<ScopeKind>> {
        &mut self.scopes[Self::GLOBAL_SCOPE_INDEX].scope
    }
}


































