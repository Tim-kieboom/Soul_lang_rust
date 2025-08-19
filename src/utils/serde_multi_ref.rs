use std::{collections::HashMap, marker::PhantomData, ops::{Deref, DerefMut}, sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard}};
use my_macros::CloneWithPool;
use serde::{Deserialize, Serialize};

use crate::steps::step_interfaces::i_parser::{abstract_syntax_tree::{abstract_syntax_tree::GlobalNode, spanned::Spanned, staments::{enum_likes::{InnerEnumDecl, InnerTypeEnumDecl, InnerUnionDecl}, function::{FnDeclKind, InnerFunctionSignature, InnerLambdaSignature}, objects::{InnerClassDecl, InnerStructDecl, InnerTraitDecl}, statment::{Block, VariableDecl}}}, scope::TypeDefed};


#[derive(Debug, Clone, CloneWithPool, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MultiRefId(pub usize);

#[derive(Debug, Clone, CloneWithPool, PartialEq, Serialize, Deserialize)]
pub struct MultiRef<T: PoolValueConvertable> {
    id: MultiRefId,

    #[serde(skip)]
    _t: PhantomData<T>,
}

pub trait MultiRefPoolClone {
    fn clone_to_pool(&self, src_pool: &MultiRefPool, dest_pool: &mut MultiRefPool) -> Self;
}

impl<T: PoolValueConvertable> MultiRef<T> {
    pub fn new(value: T, pool: &mut MultiRefPool) -> Self {
        pool.add(value)
    }

    pub fn borrow(&self, pool: &MultiRefPool) -> MultiRefReadGuard<T> {
        pool.read_value(&self.id).expect(format!("MultiRef(id: {}) not found in ref_pool", self.id.0).as_str())
    }

    pub fn try_borrow(&self, pool: &MultiRefPool) -> Result<MultiRefReadGuard<T>, String> {
        pool.read_value(&self.id).ok_or(format!("MultiRef(id: {}) not found in ref_pool", self.id.0))
    }

    pub fn borrow_mut(&mut self, pool: &mut MultiRefPool) -> MultiRefWriteGuard<T> {
        pool.write_value(&self.id).expect(format!("MultiRef(id: {}) not found in ref_pool", self.id.0).as_str())
    }

    pub fn try_borrow_mut(&mut self, pool: &mut MultiRefPool) -> Result<MultiRefWriteGuard<T>, String> {
        pool.write_value(&self.id).ok_or(format!("MultiRef(id: {}) not found in ref_pool", self.id.0))
    }

    pub unsafe fn consume(self, pool: &mut MultiRefPool) -> T {
        pool.consume_value(&self.id)
            .inspect_err(|err| panic!("{err}"))
            .unwrap_unchecked()
            .expect(format!("MultiRef(id: {}) not found in ref_pool", self.id.0).as_str())
    } 

    pub fn try_consume(self, pool: &mut MultiRefPool) -> Result<T, String> {
        pool.consume_value(&self.id)?
            .ok_or(format!("MultiRef(id: {}) not found in ref_pool", self.id.0))
    }
}

impl<T: PoolValueConvertable> MultiRefPoolClone for MultiRef<T> {
    fn clone_to_pool(&self, src_pool: &MultiRefPool, dest_pool: &mut MultiRefPool) -> Self {
        Self::new(self.borrow(src_pool).clone(), dest_pool)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
///store that keeps all the shared refs in the AST and scopes
pub struct MultiRefPool {
    store: HashMap<MultiRefId, Arc<RwLock<PoolValue>>>,
    next_id: MultiRefId,
    empty_space_stack: Vec<MultiRefId>,
}

impl MultiRefPool {
    pub fn new() -> Self {
        Self { store: HashMap::new(), next_id: MultiRefId(0), empty_space_stack: vec![] }
    }

    fn add<T: PoolValueConvertable>(&mut self, value: T) -> MultiRef<T> {
        let id = if let Some(id) = self.empty_space_stack.pop() {
            id
        }
        else {
            let id = self.next_id;
            self.next_id.0 += 1;
            id
        };

        self.store.insert(id, Arc::new(RwLock::new(value.to_pool_value())));
        MultiRef{id, _t: PhantomData}
    }

    fn read_value<T: PoolValueConvertable>(&self, id: &MultiRefId) -> Option<MultiRefReadGuard<T>> {
        let arc = self.store.get(id)?;
        MultiRefReadGuard::new(arc.clone())
    }

    fn write_value<T: PoolValueConvertable>(&mut self, id: &MultiRefId) -> Option<MultiRefWriteGuard<T>> {
        let arc = self.store.get(id)?;
        MultiRefWriteGuard::new(arc.clone())
    }

    fn consume_value<T: PoolValueConvertable>(&mut self, id: &MultiRefId) -> Result<Option<T>, String> {
        let arc = match self.store.remove(id) {
            Some(val) => val,
            None => return Ok(None),
        };
        
        let value = Arc::try_unwrap(arc)
            .or(Err("failed to unwrap Arc (meaning Arc has more then 1 strong refrence alive)"))?
            .into_inner()
            .map_err(|err| err.to_string())?;
        
        self.empty_space_stack.push(*id);
        Ok(Some(T::from_pool_value(value)))
    }
}

///trait used in MultiRef to go to and from PoolValue enum
pub trait PoolValueConvertable: Clone + Serialize {
    fn is_valid_pool_value(val: &PoolValue) -> bool;
    fn from_pool_value(val: PoolValue) -> Self;
    fn from_pool_value_ref(val: &PoolValue) -> &Self;
    fn from_pool_value_mut(val: &mut PoolValue) -> &mut Self;
    fn to_pool_value(self) -> PoolValue;
}

pub trait CloneWithPool {
    fn clone_change_ref_pool(&self, src_pool: &MultiRefPool, dst_pool: &mut MultiRefPool) -> Self;
}

macro_rules! define_pool_value {
    (
        $(
            $variant:ident => $ty:ty
        ),* $(,)?
    ) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum PoolValue {
            $(
                $variant($ty),
            )*
        }

        $(
            impl PoolValueConvertable for $ty {
                fn is_valid_pool_value(val: &PoolValue) -> bool {
                    match val {
                        PoolValue::$variant(_) => true,
                        _ => false,
                    }
                }
                fn from_pool_value(val: PoolValue) -> Self {
                    match val {
                        PoolValue::$variant(val) => val,
                        _ => panic!("PoolValue from_pool_value called from wrong type"),
                    }
                }
                fn from_pool_value_ref(val: &PoolValue) -> &Self {
                    match val {
                        PoolValue::$variant(val) => val,
                        _ => panic!("PoolValue from_pool_value called from wrong type"),
                    }
                }
                fn from_pool_value_mut(val: &mut PoolValue) -> &mut Self {
                    match val {
                        PoolValue::$variant(val) => val,
                        _ => panic!("PoolValue from_pool_value called from wrong type"),
                    }
                }
                fn to_pool_value(self) -> PoolValue {
                    PoolValue::$variant(self)
                }
            }
        )*
    };
}

define_pool_value!{
    FuncSig => Spanned<InnerFunctionSignature>,   
    OverFuncs => Vec<FnDeclKind>,
    Struct => InnerStructDecl,
    Var => VariableDecl,
    Block => Spanned<Block>,
    GlobalNodes => Vec<GlobalNode>,
    Class => InnerClassDecl,
    TypeEnum => InnerTypeEnumDecl,
    Union => InnerUnionDecl,
    Enum => InnerEnumDecl,
    TypeDefed => TypeDefed,
    Trait => InnerTraitDecl,
    Lambda => InnerLambdaSignature,
}

pub struct MultiRefReadGuard<T: PoolValueConvertable> {
    _lock_owner: Arc<RwLock<PoolValue>>,
    guard: RwLockReadGuard<'static, PoolValue>,
    _t: PhantomData<T>,
}

pub struct MultiRefWriteGuard<T: PoolValueConvertable> {
    _lock_owner: Arc<RwLock<PoolValue>>,
    guard: RwLockWriteGuard<'static, PoolValue>,
    _t: PhantomData<T>,
}

impl<T: PoolValueConvertable> MultiRefReadGuard<T> {
    pub fn new(lock: Arc<RwLock<PoolValue>>) -> Option<Self> {

        let guard = lock.read().ok()?;
        if !T::is_valid_pool_value(&guard) {
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

impl<T: PoolValueConvertable> MultiRefWriteGuard<T> {
    pub fn new(lock: Arc<RwLock<PoolValue>>) -> Option<Self> {

        let guard = lock.write().ok()?;
        if !T::is_valid_pool_value(&guard) {
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

impl<T: PoolValueConvertable> Deref for MultiRefReadGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.borrow()
    }
}

impl<T: PoolValueConvertable> Deref for MultiRefWriteGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.borrow()
    }
}

impl<T: PoolValueConvertable> DerefMut for MultiRefWriteGuard<T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        self.borrow_mut()
    }
}






