use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone)]
pub struct MultiRef<T: Serialize> {
    inner: Arc<RwLock<T>>
}

impl<T: Serialize> Serialize for MultiRef<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        self.borrow().serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for MultiRef<T>
where
    T: Deserialize<'de> + Serialize,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Ok(MultiRef { inner: Arc::new(RwLock::new(value)) })
    }
}


impl<T: Serialize> MultiRef<T> {
    pub fn new(var: T) -> Self {
        Self { inner: Arc::new(RwLock::new(var)) }
    }

    pub fn borrow(&self) -> RwLockReadGuard<T> {
        self.inner.read().unwrap()
    } 

    pub fn borrow_mut(&self) -> RwLockWriteGuard<T> {
        self.inner.write().unwrap()
    } 

    pub fn consume(self) -> T {
        unsafe { Arc::try_unwrap(self.inner)
            .inspect_err(|_| panic!("internal error consumed nodeRef without this ref being the only owner")).unwrap_unchecked()
            .into_inner().unwrap() }
    }
}

impl<T: Serialize> PartialEq for MultiRef<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}








































