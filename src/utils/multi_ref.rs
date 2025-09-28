pub mod arc {
    use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};


    #[derive(Debug, Clone)]
    pub struct MultiRef<T> {
        inner: Arc<RwLock<T>>,
    }

    impl<T> MultiRef<T> {
        pub fn new(value: T) -> Self {
            Self { 
                inner: Arc::new(RwLock::new(value)),
            }
        }
    
        pub fn borrow(&self) -> RwLockReadGuard<'_, T> {
            self.inner.read().unwrap()
        }
    
        pub fn borrow_mut(&mut self) -> RwLockWriteGuard<'_, T> {
            self.inner.write().unwrap()
        }
    
        pub fn try_consume(self) -> Result<T, Self> {
            Arc::try_unwrap(self.inner)
                .map_err(|inner| Self{inner})
                .map(|lock| lock.into_inner().expect("lock is poisoned"))
        }
    }
}

pub mod rc {
    use std::{cell::{Ref, RefCell, RefMut}, rc::Rc};

    #[derive(Debug, Clone)]
    pub struct MultiRef<T> {
        inner: Rc<RefCell<T>>
    }

    impl<T> MultiRef<T> {
        pub fn new(value: T) -> Self {
            Self { inner: Rc::new(RefCell::new(value)) }
        }
    
        pub fn borrow(&self) -> Ref<'_, T> {
            self.inner.borrow()
        }
    
        pub fn borrow_mut(&mut self) -> RefMut<'_, T> {
            self.inner.borrow_mut()
        }
    
        pub fn try_consume(self) -> Result<T, Self> {
            Rc::try_unwrap(self.inner)
                .map_err(|inner| Self{inner})
                .map(|lock| lock.into_inner())
        }
    }
}












