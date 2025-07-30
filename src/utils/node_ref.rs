use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug, Clone)]
pub struct NodeRef<T> {
    inner: Arc<RwLock<T>>
}

impl<T> NodeRef<T> {
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

impl<T> PartialEq for NodeRef<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}








































