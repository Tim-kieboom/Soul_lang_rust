use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use my_macros::CloneWithPool;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, marker::PhantomData, ops::{Deref, DerefMut}};
use crate::steps::step_interfaces::i_parser::{abstract_syntax_tree::{abstract_syntax_tree::GlobalNode, spanned::Spanned, staments::{enum_likes::{InnerEnumDecl, InnerTypeEnumDecl, InnerUnionDecl}, function::{FnDeclKind, InnerFunctionSignature, InnerLambdaSignature}, objects::{InnerClassDecl, InnerStructDecl, InnerTraitDecl}, statment::{Block, VariableDecl}}}, scope::TypeDefed};


#[derive(Debug, Clone, CloneWithPool, Default, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MultiRefId(pub usize);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiRef<T: PoolValueConvertable> {
    id: MultiRefId,

    #[serde(skip)]
    arc: Option<Arc<RwLock<PoolValue>>>,
    #[serde(skip)]
    _t: PhantomData<T>,
}

impl<T: PoolValueConvertable> PartialEq for MultiRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self._t == other._t
    }
}

pub trait MultiRefPoolClone {
    fn clone_to_pool(&self, src_pool: &MultiRefPool, dest_pool: &mut MultiRefPool) -> Self;
}

impl<T: PoolValueConvertable> MultiRef<T> {
    pub fn new(value: T, pool: &mut MultiRefPool) -> Self {
        pool.add(value)
    }

    fn err_not_found(&self) -> String {
        format!("MultiRef(id: {}) not found in ref_pool", self.id.0)
    }

    fn get_arc<'a>(&self, pool: &'a MultiRefPool) -> &'a Arc<RwLock<PoolValue>> {
        pool.store.get(&self.id).expect(&self.err_not_found())
    }

    fn try_get_arc<'a>(&self, pool: &'a MultiRefPool) -> Option<&'a Arc<RwLock<PoolValue>>> {
        pool.store.get(&self.id)
    }

    fn load_arc(&mut self, pool: &MultiRefPool) {
        if self.arc.is_none() {
            let arc = self.get_arc(pool);
            self.arc = Some(arc.clone());
        }
    }

    pub fn borrow<'a>(&self, pool: &'a MultiRefPool) -> MultiRefReadGuard<'a, T> {
        let arc = self.get_arc(pool);
        pool.read_value(&arc).expect(self.err_not_found().as_str())
    }

    pub fn try_borrow<'a>(&'a self, pool: &'a MultiRefPool) -> Result<MultiRefReadGuard<'a, T>, String> {
        let arc = self.try_get_arc(pool).ok_or(self.err_not_found())?;
        pool.read_value(&arc).ok_or(self.err_not_found())
    }    
    
    pub fn owned_borrow<'a>(&'a mut self, pool: &MultiRefPool) -> MultiRefReadGuard<'a, T> {
        self.load_arc(pool);
        pool.read_value(self.arc.as_ref().unwrap()).expect(self.err_not_found().as_str())
    }

    pub fn try_owned_borrow<'a>(&'a mut self, pool: &MultiRefPool) -> Result<MultiRefReadGuard<'a, T>, String> {
        self.load_arc(pool);
        pool.read_value(self.arc.as_ref().unwrap()).ok_or(self.err_not_found())
    }

    pub fn borrow_mut(&'_ mut self, pool: &MultiRefPool) -> MultiRefWriteGuard<'_, T> {
        self.load_arc(pool);
        pool.write_value(self.arc.as_ref().unwrap()).expect(self.err_not_found().as_str())
    }

    pub fn try_borrow_mut(&'_ mut self, pool: &MultiRefPool) -> Result<MultiRefWriteGuard<'_, T>, String> {
        self.load_arc(pool);
        pool.write_value(self.arc.as_ref().unwrap()).ok_or(self.err_not_found())
    }

    pub unsafe fn consume(self, pool: &mut MultiRefPool) -> T {
        pool.consume_value(&self.id)
            .inspect_err(|err| panic!("{err}"))
            .unwrap_unchecked()
            .expect(self.err_not_found().as_str())
    } 

    pub fn try_consume(self, pool: &mut MultiRefPool) -> Result<T, (Self, String)> {
        match pool.consume_value(&self.id) {
            Ok(val) => val,
            Err(err) => return Err((self, err)),
        }.ok_or((self, format!("MultiRef not found in ref_pool")))
    }
}

impl<T: PoolValueConvertable> MultiRefPoolClone for MultiRef<T> {
    fn clone_to_pool(&self, src_pool: &MultiRefPool, dest_pool: &mut MultiRefPool) -> Self {
        let this = self.borrow(src_pool).clone();
        dest_pool.add(this)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

        let arc = Arc::new(RwLock::new(value.to_pool_value()));
        self.store.insert(id, arc.clone());
        MultiRef{id, arc: Some(arc), _t: PhantomData}
    }

    fn read_value<'a, T: PoolValueConvertable>(&self, lock: &'a RwLock<PoolValue>) -> Option<MultiRefReadGuard<'a, T>> {
        Some(MultiRefReadGuard::new(lock))
    }

    fn write_value<'a, T: PoolValueConvertable>(&self, lock: &'a RwLock<PoolValue>) -> Option<MultiRefWriteGuard<'a, T>> {
        Some(MultiRefWriteGuard::new(lock))
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

pub struct MultiRefReadGuard<'a, T: PoolValueConvertable> {
    guard: RwLockReadGuard<'a, PoolValue>,
    _t: PhantomData<T>,
}

impl<'a, T: PoolValueConvertable> MultiRefReadGuard<'a, T> {
    pub fn new(lock: &'a RwLock<PoolValue>) -> Self {
        MultiRefReadGuard {
            guard: lock.read().unwrap(),
            _t: PhantomData,
        }
    }
}

impl<'a, T: PoolValueConvertable> Deref for MultiRefReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        T::from_pool_value_ref(&*self.guard)
    }
}

pub struct MultiRefWriteGuard<'a, T: PoolValueConvertable> {
    guard: RwLockWriteGuard<'a, PoolValue>,
    _t: PhantomData<T>,
}

impl<'a, T: PoolValueConvertable> MultiRefWriteGuard<'a, T> {
    pub fn new(lock: &'a RwLock<PoolValue>) -> Self {
        MultiRefWriteGuard {
            guard: lock.write().unwrap(),
            _t: PhantomData,
        }
    }
}

impl<'a, T: PoolValueConvertable> Deref for MultiRefWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        T::from_pool_value_ref(&*self.guard)
    }
}

impl<'a, T: PoolValueConvertable> DerefMut for MultiRefWriteGuard<'a, T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        T::from_pool_value_mut(&mut self.guard)
    }
}
