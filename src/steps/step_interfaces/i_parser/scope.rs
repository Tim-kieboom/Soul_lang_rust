use std::collections::HashMap;
use crate::{steps::step_interfaces::i_parser::abstract_syntax_tree::{soul_type::type_kind::TypeKind, statment::{ClassDecl, EnumDecl, FunctionDecl, GenericParam, InterfaceDecl, StructDecl, TraitDecl, TypeEnumDecl, UnionDecl, VariableDecl}}, utils::push::Push};

pub type ScopeStack = InnerScopeBuilder<ScopeKind>;
pub type TypeScopeStack = InnerScopeBuilder<TypeKind>;

pub type Scope = InnerScope<Vec<ScopeKind>>;
pub type TypeScope = InnerScope<TypeKind>;

#[derive(Debug, Clone)]
pub struct ScopeBuilder {
    scopes: ScopeStack,
    types: Vec<TypeScope>,
}

#[derive(Debug, Clone)]
pub struct InnerScopeBuilder<T> {
    pub scopes: Vec<InnerScope<T>>,
    pub current: usize,
}

#[derive(Debug, Clone)]
pub struct InnerScope<T> {
    pub parent_index: Option<usize>,
    pub children: Vec<usize>,
    pub self_index: usize,

    pub symbols: HashMap<String, T>,

    pub visibility_mode: ScopeVisibility,
}

impl ScopeBuilder {
    pub fn new(type_stack: TypeScopeStack) -> Self {
        Self { scopes: ScopeStack::new(), types: type_stack.scopes }
    }

    pub fn push(&mut self, parent_index: usize, scope_visability: ScopeVisibility) {
        self.scopes.push(parent_index, scope_visability);
    }

    pub fn pop(&mut self) {
        self.scopes.pop();
    }

    pub fn is_in_global(&self) -> bool {
        self.scopes.is_in_global()
    } 

    pub fn lookup_forwarded_type_kind(&self, name: &str) -> Option<&TypeKind> {
        let mut current_index = Some(self.scopes.current);

        while let Some(index) = current_index {
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
}

impl<T> InnerScopeBuilder<T> {
    pub const GLOBAL_SCOPE_INDEX: usize = 0; 

    pub fn new() -> Self {
        Self { 
            scopes: vec![InnerScope::<T>::new_global()],  
            current: Self::GLOBAL_SCOPE_INDEX 
        }
    }

    pub fn push(&mut self, parent_index: usize, scope_visability: ScopeVisibility) {
        self.current = self.scopes.len();
        self.scopes[parent_index].children.push(self.current);
        self.scopes.push(InnerScope::<T>::new_child(self.current, parent_index, scope_visability));
    }

    pub fn push_current(&mut self, scope_visability: ScopeVisibility) {
        let parent_index = self.current;
        self.current = self.scopes.len();
        self.scopes[parent_index].children.push(self.current);
        self.scopes.push(InnerScope::<T>::new_child(self.current, parent_index, scope_visability));
    }

    pub fn pop(&mut self) {
        if let Some(parent_index) = self.scopes[self.current].parent_index {
            self.current = parent_index;
        } else {
            panic!("Cannot pop the global scope");
        }
    }

    pub fn is_in_global(&self) -> bool {
        self.current == Self::GLOBAL_SCOPE_INDEX
    } 

    pub fn lookup(&self, name: &str) -> Option<&T> {
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

    pub fn insert(&mut self, name: String, kind: T) {
        self.current_mut()
            .symbols
            .insert(name, kind);
    }

    pub fn insert_to_vec<V>(&mut self, name: String, kind: V) 
    where 
        T: Push<V> + Default
    {
        self.current_mut()
            .symbols
            .entry(name)
            .or_default()
            .push(kind);
    }

    pub fn current(&self) -> &InnerScope<T> {
        &self.scopes[self.current]
    }

    pub fn current_mut(&mut self) -> &mut InnerScope<T> {
        &mut self.scopes[self.current]
    }
}

impl<T> InnerScope<T> {
    pub fn new_global() -> Self {
        Self {
            parent_index: None,
            children: vec![],
            self_index: 0,
            symbols: HashMap::new(),
            visibility_mode: ScopeVisibility::GlobalOnly, // does not matter for global
        }
    }

    pub fn new_child(self_index: usize, parent_index: usize, vis: ScopeVisibility) -> Self {
        Self {
            self_index,
            children: vec![],
            symbols: HashMap::new(),
            parent_index: Some(parent_index),
            visibility_mode: vis,
        }
    }

    pub fn get(&self, name: &str) -> Option<&T> {
        self.symbols.get(name)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeVisibility {
    All,         // Can access child -> parent -> ... -> global
    GlobalOnly,  // Can only access global scope
}

#[derive(Debug, Clone)]
pub enum ScopeKind {
    Invalid,
    Variable(VariableDecl),
    Struct(StructDecl),
    Class(ClassDecl),

    Trait(TraitDecl),
    Interface(InterfaceDecl),

    Functions(OverloadedFunctions),

    Enum(EnumDecl),
    Union(UnionDecl),
    TypeEnum(TypeEnumDecl),

    CurrentGeneric(GenericParam),
}


#[derive(Debug, Clone)]
pub struct OverloadedFunctions {
    pub functions: Vec<FunctionDecl>,
}





