use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::errors::soul_error::Result;
use std::{collections::{BTreeMap, HashMap}};
use crate::{errors::soul_error::{new_soul_error, SoulError, SoulErrorKind, SoulSpan}, steps::step_interfaces::i_parser::abstract_syntax_tree::{enum_like::{Enum, TypeEnum, Union}, expression::{Expression, Ident}, function::Function, literal::Literal, object::{Class, Struct, Trait}, soul_type::{soul_type::SoulType}, spanned::Spanned}};


type Scope = InnerScope<Vec<Spanned<ScopeKind>>>;

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct ScopeBuilder {
    scopes: Vec<Scope>,
    current: usize,
    pub global_literals: ProgramMemmory,
    pub project_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct InnerScope<T> {
    pub parent_index: Option<usize>,
    pub children: Vec<usize>,
    pub current_child: usize,
    pub self_index: usize,

    pub symbols: HashMap<String, T>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum ScopeKind {
    Class(Class),
    Trait(Trait),
    Struct(Struct),

    Variable(Variable),
    Functions(Vec<Spanned<Function>>),

    Enum(Enum),
    Union(Union),
    TypeEnum(TypeEnum),
    Type(SoulType),
    TypeDef{new_type: SoulType, of_type: SoulType},
    UseTypeDef{new_type: SoulType, of_type: SoulType},
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Variable {
    pub name: Ident,
    pub ty: SoulType,
    pub initialize_value: Option<Expression>,
}

#[derive(Debug, Hash, Clone, Copy, Serialize, Deserialize, Encode, Decode)]
pub struct ProgramMemmoryId(pub usize);

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
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

    pub fn new(project_name: String) -> Self {
        Self { 
            scopes: vec![Scope::new_global()], 
            current: Self::GLOBAL_SCOPE_INDEX, 
            global_literals: ProgramMemmory::new(),
            
            project_name,
        }
    }

    pub fn get_scopes(&self) -> &Vec<Scope> {
        &self.scopes
    }

    pub fn get_global_scope(&self) -> &Scope {
        &self.scopes[Self::GLOBAL_SCOPE_INDEX]
    }

    pub fn current_scope(&self) -> &Scope {
        &self.scopes[self.current]
    }

    pub fn current_index(&self) -> usize {
        self.current
    }

    pub fn push_scope(&mut self) {
        let parent_index = self.current;
        self.current = self.scopes.len();
        self.scopes[parent_index].children.push(self.current);
        self.scopes.push(InnerScope::new_child(self.current, parent_index));
    }

    pub fn remove_current(&mut self, span: SoulSpan) -> std::result::Result<(), SoulError> {
        let current = self.current;
        if let Some(parent_index) = self.scopes[self.current].parent_index {
            self.current = parent_index;
            let self_index = self.scopes[parent_index].children.iter().enumerate().find(|(_i, el)| **el == current).unwrap().0;
            self.scopes[parent_index].children.remove(self_index);
            Ok(())
        } 
        else {
            Err(new_soul_error(SoulErrorKind::UnexpectedEnd, Some(span), "can not remove global scope"))
        }
    }
    
    pub fn pop_scope(&mut self, span: SoulSpan) -> std::result::Result<(), SoulError> {
        if let Some(parent_index) = self.scopes[self.current].parent_index {
            self.current = parent_index;
            Ok(())
        } 
        else {
            Err(new_soul_error(SoulErrorKind::UnexpectedEnd, Some(span), "can not remove global scope"))
        }
    }

    pub fn is_in_global(&self) -> bool {
        self.current == Self::GLOBAL_SCOPE_INDEX
    }

    pub fn insert(&mut self, name: String, kind: ScopeKind, span: SoulSpan) -> Result<()> {
        use std::mem::discriminant;

        let entry = self.current_mut()
            .symbols
            .entry(name.clone())
            .or_default();

        if entry.iter().any(|el| discriminant(&el.node) == discriminant(&kind)) {
            
            return Err(new_soul_error(
                SoulErrorKind::InvalidName, 
                Some(span),
                format!("'{}' already exists", name),
            ))
        }

        entry.push(Spanned::new(kind, span));
        
        Ok(())
    } 

    pub fn insert_function(&mut self, function: Function, span: SoulSpan) {
        let scope = self.current_mut()
            .symbols
            .entry(function.signature.name.0.clone())
            .or_default();

        if let Some(kind) = scope.iter_mut().find(|el| matches!(el.node, ScopeKind::Functions(_))) {
            
            if let ScopeKind::Functions(functions) = &mut kind.node {
                functions.push(Spanned::new(function ,span));
            }
            else {unreachable!()}
        }
        else {
            scope.push(Spanned::new(ScopeKind::Functions(vec![Spanned::new(function, span)]), span));
        }
    } 

    pub fn insert_global(&mut self, name: String, kind: ScopeKind, span: SoulSpan) {
        self.global_mut()
            .symbols
            .entry(name)
            .or_default()
            .push(Spanned::new(kind, span));
    }

    pub fn insert_global_function(&mut self, function: Function, span: SoulSpan) {
        let scope = self.global_mut()
            .symbols
            .entry(function.signature.name.0.clone())
            .or_default();

        if let Some(kind) = scope.iter_mut().find(|el| matches!(el.node, ScopeKind::Functions(_))) {
            
            if let ScopeKind::Functions(functions) = &mut kind.node {
                functions.push(Spanned::new(function ,span));
            }
            else {unreachable!()}
        }
        else {
            scope.push(Spanned::new(ScopeKind::Functions(vec![Spanned::new(function, span)]), span));
        }
    }

    pub fn current(&self) -> &Scope {
        &self.scopes[self.current]
    }

    pub fn current_mut(&mut self) -> &mut Scope {
        &mut self.scopes[self.current]
    }

    pub fn global_mut(&mut self) -> &mut Scope {
        &mut self.scopes[Self::GLOBAL_SCOPE_INDEX]
    }

    pub fn __consume_to_tuple(self) -> (Vec<Scope>, usize, ProgramMemmory, String) {
        (
            self.scopes,
            self.current,
            self.global_literals,
            self.project_name,
        )
    }
}

impl<T> InnerScope<T> {
    pub fn new_global() -> Self {
        Self {
            parent_index: None,
            children: vec![],
            current_child: 0,
            self_index: 0,
            symbols: HashMap::new(),
        }
    }

    pub fn new_child(self_index: usize, parent_index: usize) -> Self {
        Self {
            self_index,
            children: vec![],
            current_child: 0,
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

















