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
    pub scopes: Vec<InnerScope<Vec<ScopeKind>>>,
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

    pub fn next_sibling(&mut self) -> bool {
        self.scopes.next_sibling()
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
        &self.scopes.scopes[InnerScopeVisitor::GLOBAL_SCOPE_INDEX]
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
            scopes,
            current: Self::GLOBAL_SCOPE_INDEX,
        }
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
            return false;
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

    pub fn flat_lookup(&self, name: &str) -> Option<&Vec<ScopeKind>> {
        let scope = &self.scopes[self.current];

        if let Some(kinds) = scope.get(name) {
            return Some(kinds);
        }

        None
    }
        
    pub fn flat_lookup_mut(&mut self, name: &str) -> Option<&mut Vec<ScopeKind>> {
        let scope = &mut self.scopes[self.current];

        if let Some(kinds) = scope.get_mut(name) {
            return Some(kinds);
        }

        None
    }

    pub fn lookup(&self, name: &str) -> Option<&Vec<ScopeKind>> {
        let mut current_index = Some(self.current);

        while let Some(index) = current_index {
            let scope = &self.scopes[index];

            if let Some(kinds) = scope.get(name) {
                return Some(kinds);
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

    pub fn current(&self) -> &InnerScope<Vec<ScopeKind>> {
        &self.scopes[self.current]
    }

    pub fn current_mut(&mut self) -> &mut InnerScope<Vec<ScopeKind>> {
        &mut self.scopes[self.current]
    }

    pub fn global_mut(&mut self) -> &mut InnerScope<Vec<ScopeKind>> {
        &mut self.scopes[Self::GLOBAL_SCOPE_INDEX]
    }
}


































