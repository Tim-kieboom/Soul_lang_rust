use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::steps::step_interfaces::i_parser::{abstract_syntax_tree::{expression::{Expression, Ident}, function::{Function, FunctionSignature}, generic::GenericParameter, soul_type::soul_type::SoulType, spanned::Spanned, statement::UseBlock}, scope_builder::ScopeId};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Struct {
    pub name: Ident,
    pub generics: Vec<GenericParameter>,
    pub fields: Vec<Spanned<Field>>,
    pub scope_id: ScopeId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Class {
    pub name: Ident,
    pub generics: Vec<GenericParameter>,
    pub implements: Vec<SoulType>,
    pub children: Vec<ClassChild>,
    pub scope_id: ScopeId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Trait {
    pub signature: TraitSignature,
    pub methodes: Vec<Spanned<FunctionSignature>>,
    pub scope_id: ScopeId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct TraitSignature {
    pub name: Ident,
    pub generics: Vec<GenericParameter>,
    pub implements: Vec<SoulType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum ClassChild {
    Field(Spanned<Field>),
    Methode(Spanned<Function>),
    ImplBlock(Spanned<UseBlock>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Field {
    pub name: Ident,
    pub ty: SoulType,
    pub default_value: Option<Expression>,
    pub vis: FieldAccess
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct FieldAccess {
    /// None = use default (e.g. pub)
    pub get: Option<Visibility>, 
    // None = disallow set
    pub set: Option<Visibility>, 
}

impl FieldAccess {
    pub fn new_public() -> Self {
        Self{ get: Some(Visibility::Public), set: Some(Visibility::Public)}
    }

    pub fn new_private() -> Self {
        Self{ get: Some(Visibility::Private), set: Some(Visibility::Private)}
    }
}

impl Default for FieldAccess  {
    fn default() -> Self {
        Self { get: None, set: None }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum Visibility {
    Public,
    Private,
}

impl ClassChild {
    pub fn get_scope_id(&self) -> Option<ScopeId> {
        
        match self {
            ClassChild::Field(_) => None,
            ClassChild::Methode(spanned) => Some(spanned.node.block.scope_id),
            ClassChild::ImplBlock(spanned) => Some(spanned.node.block.scope_id),
        } 
    }
}












