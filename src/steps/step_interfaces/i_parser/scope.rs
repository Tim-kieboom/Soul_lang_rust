use std::collections::{BTreeMap, HashMap};
use crate::{errors::soul_error::{new_soul_error, SoulErrorKind}, steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::Ident, literal::Literal, soul_type::type_kind::TypeKind, statment::{ClassDecl, EnumDecl, FnDecl, GenericParam, NodeRef, StructDecl, TraitDecl, TypeEnumDecl, UnionDecl, VariableRef}}, utils::push::Push};

pub type ScopeStack = InnerScopeBuilder<Vec<ScopeKind>>;
pub type TypeScopeStack = InnerScopeBuilder<TypeKind>;

pub type Scope = InnerScope<Vec<ScopeKind>>;
pub type TypeScope = InnerScope<TypeKind>;

#[derive(Debug, Clone)]
pub struct ScopeBuilder {
    scopes: ScopeStack,
    types: Vec<TypeScope>,
    pub global_literal: ProgramMemmory,
}

#[derive(Debug, Hash, Clone, Copy, )]
pub struct ProgramMemmoryId(pub usize);


#[derive(Debug, Clone)]
pub struct ProgramMemmory {
    pub store: BTreeMap<Literal, ProgramMemmoryId>,
    pub last_id: ProgramMemmoryId,
}
impl ProgramMemmory {
    pub fn new() -> Self {
        Self { store: BTreeMap::new(), last_id: ProgramMemmoryId(0) }
    }

    pub fn insert(&mut self, entry: Literal) -> ProgramMemmoryId {
        let id = self.last_id;
        self.last_id.0 += 1;
        self.store.insert(entry, id);
        return id;
    }

    pub fn to_program_memory_name(this: &ProgramMemmoryId) -> Ident {
        Ident(format!("__soul_mem_{}", this.0))
    }
}


#[derive(Debug, Clone)]
pub struct InnerScopeBuilder<T> {
    pub scopes: Vec<InnerScope<T>>,
    pub current: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InnerScope<T> {
    pub parent_index: Option<usize>,
    pub children: Vec<usize>,
    pub self_index: usize,

    pub symbols: HashMap<String, T>,

    pub visibility_mode: ScopeVisibility,
}

impl ScopeBuilder {
    pub fn new(type_stack: TypeScopeStack) -> Self {
        Self { scopes: ScopeStack::new(), global_literal: ProgramMemmory::new(), types: type_stack.scopes }
    }

    pub fn get_scopes(&self) -> &InnerScopeBuilder<Vec<ScopeKind>> {
        &self.scopes
    }

    pub fn get_types(&self) -> &Vec<InnerScope<TypeKind>> {
        &self.types
    }

    pub fn current_scope(&self) -> &InnerScope<Vec<ScopeKind>> {
        &self.scopes.scopes[self.scopes.current]
    }

    pub fn push(&mut self, scope_visability: ScopeVisibility) {
        self.scopes.push_current(scope_visability);
    }

    pub fn push_from(&mut self, parent_index: usize, scope_visability: ScopeVisibility) {
        self.scopes.push(parent_index, scope_visability);
    }

    pub fn pop(&mut self) {
        self.scopes.pop();
    }

    pub fn is_in_global(&self) -> bool {
        self.scopes.is_in_global()
    } 

    ///only looks in current scope
    pub fn flat_lookup(&self, name: &str) -> Option<&Vec<ScopeKind>> {
        self.scopes.flat_lookup(name)
    }
    
    ///looks in current scope and parent scopes of ScopeVisibilty is All
    pub fn lookup(&self, name: &str) -> Option<&Vec<ScopeKind>> {
        self.scopes.lookup(name)
    }

    pub fn add_function(&mut self, fn_decl: FnDecl) {
        
        if let Some(kinds) = self.scopes.flat_lookup_mut(&fn_decl.signature.name.0) {
            
            let possible_funcs = kinds.iter_mut().find(|kind| matches!(kind, ScopeKind::Functions(..)));
            if let Some(ScopeKind::Functions(funcs)) = possible_funcs {
                funcs.borrow_mut().push(fn_decl);
            }
            else {
                self.scopes.insert(
                    fn_decl.signature.name.0.clone(), 
                    vec![ScopeKind::Functions(OverloadedFunctions::new(vec![fn_decl]))]
                );
            }
        }
        else {
            self.scopes.insert(
                fn_decl.signature.name.0.clone(), 
                vec![ScopeKind::Functions(OverloadedFunctions::new(vec![fn_decl]))]
            );
        }
    } 

    pub fn insert(&mut self, name: String, kind: ScopeKind) {
        self.scopes.insert_to_vec(name, kind)
    } 

    pub fn insert_type(&mut self, name: String, kind: TypeKind) -> std::result::Result<(), String> {
        let types = &mut self.types[self.scopes.current];

        if types.get(&name).is_some() {
            return Err(format!("type: '{}' already exists", name));
        }

        types.symbols.insert(name, kind);
        Ok(())
    }  

    pub fn insert_global(&mut self, name: String, kind: ScopeKind) {
        self.scopes.insert_global_to_vec(name, kind)
    }

    pub fn lookup_type(&self, name: &str) -> Option<&TypeKind> {
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

    pub fn flat_lookup(&self, name: &str) -> Option<&T> {
        let scope = &self.scopes[self.current];

        if let Some(kinds) = scope.get(name) {
            return Some(kinds);
        }

        None
    }
        
    pub fn flat_lookup_mut(&mut self, name: &str) -> Option<&mut T> {
        let scope = &mut self.scopes[self.current];

        if let Some(kinds) = scope.get_mut(name) {
            return Some(kinds);
        }

        None
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

    pub fn insert_global(&mut self, name: String, kind: T) {
        self.global_mut()
            .symbols
            .insert(name, kind);
    }

    pub fn insert_global_to_vec<V>(&mut self, name: String, kind: V) 
    where 
        T: Push<V> + Default
    {
        self.global_mut()
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

    pub fn global_mut(&mut self) -> &mut InnerScope<T> {
        &mut self.scopes[Self::GLOBAL_SCOPE_INDEX]
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

    pub fn get_mut(&mut self, name: &str) -> Option<&mut T> {
        self.symbols.get_mut(name)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeVisibility {
    All,         // Can access child -> parent -> ... -> global
    GlobalOnly,  // Can only access global scope
}

#[derive(Debug, Clone)]
pub enum ScopeKind {
    Invalid(),
    Variable(VariableRef),
    Struct(StructDecl),
    Class(ClassDecl),

    Trait(TraitDecl),

    Functions(OverloadedFunctions),

    Enum(EnumDecl),
    Union(UnionDecl),
    TypeEnum(TypeEnumDecl),
}

pub type OverloadedFunctions = NodeRef<Vec<FnDecl>>;





































