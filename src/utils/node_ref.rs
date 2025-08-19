use std::{collections::HashMap, marker::PhantomData, ops::{Deref, DerefMut}, sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard}};
use serde::{Deserialize, Serialize};

use crate::steps::step_interfaces::i_parser::{abstract_syntax_tree::{abstract_syntax_tree::GlobalNode, spanned::Spanned, staments::{enum_likes::{InnerEnumDecl, InnerTypeEnumDecl, InnerUnionDecl}, function::{FnDeclKind, InnerFunctionSignature, InnerLambdaSignature}, objects::{InnerClassDecl, InnerStructDecl, InnerTraitDecl, TraitDeclRef}, statment::{Block, VariableDecl}}}, scope::TypeDefed};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MultiRefId(pub u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoolValue {
    FuncSig(Spanned<InnerFunctionSignature>),   
    OverFuncs(Vec<FnDeclKind>),
    Struct(InnerStructDecl),
    Var(VariableDecl),
    Block(Spanned<Block>),
    GlobalNodes(Vec<GlobalNode>),
    Class(InnerClassDecl),
    TypeEnum(InnerTypeEnumDecl),
    Union(InnerUnionDecl),
    Enum(InnerEnumDecl),
    TypeDefed(TypeDefed),
    Trait(InnerTraitDecl),
    Lambda(InnerLambdaSignature),
}

///trait used in MultiRef to go to and from PoolValue enum
pub trait FromPoolValue: Clone {
    fn is_from_pool_value(from: &PoolValue) -> bool;
    fn from_pool_value_mut(from: &mut PoolValue) -> &mut Self;
    fn from_pool_value_ref(from: &PoolValue) -> &Self;
    fn to_pool_value(self) -> PoolValue;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiRef<T: FromPoolValue> {
    id: MultiRefId,

    #[serde(skip)]
    _t: PhantomData<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
///store that keeps all the shared refs in the AST and scopes
pub struct MultiRefPool {
    store: HashMap<MultiRefId, Arc<RwLock<PoolValue>>>,
    next_id: u64,
}

impl MultiRefPool {
    pub fn new() -> Self {
        Self { store: HashMap::new(), next_id: 0 }
    }
    
    fn push<T: FromPoolValue>(&mut self, value: T) -> MultiRefId {
        let id = MultiRefId(self.next_id);
        self.next_id += 1;

        self.store.insert(id, Arc::new(RwLock::new(value.to_pool_value())));
        id
    }

    fn get(&self, id: MultiRefId) -> Option<Arc<RwLock<PoolValue>>> {
        self.store.get(&id).cloned()
    }

    fn remove(&mut self, id: MultiRefId) -> Option<Arc<RwLock<PoolValue>>> {
        self.store.remove(&id)
    }
}

impl<T: FromPoolValue> MultiRef<T> {
    pub fn new(var: T, pool: &mut MultiRefPool) -> Self {
        let id = pool.push(var);
        Self{id, _t: PhantomData }
    }

    pub fn borrow(&self, pool: &MultiRefPool) -> MultiRefReadGuard<T> {
        MultiRefReadGuard::new(pool.get(self.id).expect(format!("MultiRef of id: '{}' was removed before end of lifetime", self.id.0).as_str()).clone()).unwrap()
    } 

    pub fn borrow_mut(&self, pool: &mut MultiRefPool) -> MultiRefWriteGuard<T> {
        MultiRefWriteGuard::new(pool.get(self.id).unwrap().clone()).unwrap()
    } 

    /// consumes the MultiRef BUT DOES NOT CONSUME THE SHARED RESOURCE it clones it
    pub fn to_owned(self, pool: &MultiRefPool) -> T {
        self.borrow(&pool).borrow().clone()
    }

    /// removes the SHARED RESOURCE returns the value
    pub unsafe fn consume(&self, pool: &mut MultiRefPool) -> T {
        let guard = MultiRefReadGuard::<T>::new(pool.remove(self.id).expect("MultiRef of id: '{}' was removed before end of lifetime")).unwrap();
        guard.borrow().clone()
    }
}

pub struct MultiRefReadGuard<T: FromPoolValue> {
    _lock_owner: Arc<RwLock<PoolValue>>,
    guard: RwLockReadGuard<'static, PoolValue>,
    _t: PhantomData<T>,
}

pub struct MultiRefWriteGuard<T: FromPoolValue> {
    _lock_owner: Arc<RwLock<PoolValue>>,
    guard: RwLockWriteGuard<'static, PoolValue>,
    _t: PhantomData<T>,
}

impl<T: FromPoolValue> MultiRefReadGuard<T> {
    pub fn new(lock: Arc<RwLock<PoolValue>>) -> Option<Self> {

        let guard = lock.read().ok()?;
        if !T::is_from_pool_value(&guard) {
            return None;
        }

        let guard = unsafe {
            std::mem::transmute::<
                RwLockReadGuard<'_, PoolValue>,
                RwLockReadGuard<'static, PoolValue>,
            >(guard)
        };

        Some(Self {
            _lock_owner: lock,
            guard,
            _t: PhantomData,
        })
    }

    fn borrow(&self) -> &T {
        T::from_pool_value_ref(&self.guard)
    }
}

impl<T: FromPoolValue> MultiRefWriteGuard<T> {
    pub fn new(lock: Arc<RwLock<PoolValue>>) -> Option<Self> {

        let guard = lock.write().ok()?;
        if !T::is_from_pool_value(&guard) {
            return None;
        }

        let guard = unsafe {
            std::mem::transmute::<
                RwLockWriteGuard<'_, PoolValue>,
                RwLockWriteGuard<'static, PoolValue>,
            >(guard)
        };

        Some(Self {
            _lock_owner: lock,
            guard,
            _t: PhantomData,
        })
    }

    fn borrow(&self) -> &T {
        T::from_pool_value_ref(&self.guard)
    }

    fn borrow_mut(&mut self) -> &mut T {
        T::from_pool_value_mut(&mut self.guard)
    }
}

impl<T: FromPoolValue> Deref for MultiRefReadGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.borrow()
    }
}

impl<T: FromPoolValue> Deref for MultiRefWriteGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.borrow()
    }
}

impl<T: FromPoolValue> DerefMut for MultiRefWriteGuard<T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        self.borrow_mut()
    }
}





























