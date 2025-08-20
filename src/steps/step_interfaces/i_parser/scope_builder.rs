use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::{errors::soul_error::{new_soul_error, SoulError, SoulErrorKind, SoulSpan}, steps::step_interfaces::i_parser::abstract_syntax_tree::{enum_like::{Enum, TypeEnum, Union}, expression::{Expression, Ident}, function::Function, literal::Literal, object::{Class, Struct, Trait}, soul_type::soul_type::SoulType}};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeBuilder {
    scopes: Vec<InnerScope<ScopeKind>>,
    current: usize,
    pub global_literals: ProgramMemmory,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerScope<T> {
    pub parent_index: Option<usize>,
    pub children: Vec<usize>,
    pub self_index: usize,

    pub symbols: HashMap<String, T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScopeKind {
    Class(Class),
    Trait(Trait),
    Struct(Struct),

    Variable(Variable),
    Functions(Vec<Function>),

    Enum(Enum),
    Union(Union),
    TypeEnum(TypeEnum),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variable {
    pub name: Ident,
    pub ty: SoulType,
    pub initialize_value: Option<Expression>,
}

#[derive(Debug, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct ProgramMemmoryId(pub usize);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramMemmory {
    pub store: BTreeMap<Literal, ProgramMemmoryId>,
    pub last_id: ProgramMemmoryId,
}
impl ProgramMemmory {
    pub fn new() -> Self {
        Self { store: BTreeMap::new(), last_id: ProgramMemmoryId(0) }
    }

    pub fn insert(&mut self, entry: Literal) -> ProgramMemmoryId {
        if let Some(id) = self.store.get(&entry) {
            return *id;
        }
        
        let id = self.last_id;
        self.last_id.0 += 1;
        self.store.insert(entry, id);
        return id;
    }

    pub fn to_program_memory_name(this: &ProgramMemmoryId) -> Ident {
        Ident(format!("__soul_mem_{}", this.0))
    }
}

impl ScopeBuilder {

    const GLOBAL_SCOPE_INDEX: usize = 0;

    pub fn new() -> Self {
        Self { 
            scopes: Vec::new(), 
            current: Self::GLOBAL_SCOPE_INDEX, 
            global_literals: ProgramMemmory::new(),
        }
    }

    pub fn get_scopes(&self) -> &Vec<InnerScope<ScopeKind>> {
        &self.scopes
    }

    pub fn get_global_scope(&self) -> &InnerScope<ScopeKind>{
        &self.scopes[Self::GLOBAL_SCOPE_INDEX]
    }

    pub fn current_scope(&self) -> &InnerScope<ScopeKind> {
        &self.scopes[self.current]
    }

    pub fn current_index(&self) -> usize {
        self.current
    }

    pub fn push(&mut self) {
        let parent_index = self.current;
        self.current = self.scopes.len();
        self.scopes[parent_index].children.push(self.current);
        self.scopes.push(InnerScope::new_child(self.current, parent_index));
    }

    pub fn remove_current(&mut self, span: SoulSpan) -> Result<(), SoulError> {
        let current = self.current;
        if let Some(parent_index) = self.scopes[self.current].parent_index {
            self.current = parent_index;
            let self_index = self.scopes[parent_index].children.iter().enumerate().find(|(_i, el)| **el == current).unwrap().0;
            self.scopes[parent_index].children.remove(self_index);
            Ok(())
        } 
        else {
            Err(new_soul_error(SoulErrorKind::UnexpectedEnd, span, "can not remove global scope"))
        }
    }
    
    pub fn pop(&mut self, span: SoulSpan) -> Result<(), SoulError> {
        if let Some(parent_index) = self.scopes[self.current].parent_index {
            self.current = parent_index;
            Ok(())
        } 
        else {
            Err(new_soul_error(SoulErrorKind::UnexpectedEnd, span, "can not remove global scope"))
        }
    }

    pub fn is_in_global(&self) -> bool {
        self.current == Self::GLOBAL_SCOPE_INDEX
    }

    pub fn insert(&mut self, name: String, kind: ScopeKind) {
        self.current_mut()
            .symbols
            .insert(name, kind);
    } 

    pub fn insert_global(&mut self, name: String, kind: ScopeKind) {
        self.global_mut()
            .symbols
            .insert(name, kind);
    }

        pub fn current(&self) -> &InnerScope<ScopeKind> {
        &self.scopes[self.current]
    }

    pub fn current_mut(&mut self) -> &mut InnerScope<ScopeKind> {
        &mut self.scopes[self.current]
    }

    pub fn global_mut(&mut self) -> &mut InnerScope<ScopeKind> {
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
        }
    }

    pub fn new_child(self_index: usize, parent_index: usize) -> Self {
        Self {
            self_index,
            children: vec![],
            symbols: HashMap::new(),
            parent_index: Some(parent_index),
        }
    }

    pub fn get(&self, name: &str) -> Option<&T> {
        self.symbols.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut T> {
        self.symbols.get_mut(name)
    }

}

















