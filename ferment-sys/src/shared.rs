use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub trait SharedAccess {
    type Item;
    type ImmutableAccess: ?Sized;
    type MutableAccess: ?Sized;

    fn clone_container(&self) -> Self where Self: Sized;

    fn access<F, R>(&self, f: F) -> R
        where
            F: FnOnce(&Self::ImmutableAccess) -> R;

    #[allow(unused)]
    fn access_mut<F, R>(&self, f: F) -> R where F: FnOnce(&Self::MutableAccess) -> R;
}
impl<T: 'static> SharedAccess for Rc<RefCell<T>> {
    type Item = T;
    type ImmutableAccess = Ref<'static, T>;
    type MutableAccess = RefMut<'static, T>;

    fn clone_container(&self) -> Self {
        Rc::clone(self)
    }

    fn access<F, R>(&self, f: F) -> R where F: FnOnce(&Self::ImmutableAccess) -> R {
        let borrowed = self.borrow();
        f(unsafe { std::mem::transmute::<&Ref<T>, &Ref<'static, T>>(&borrowed) })
    }

    fn access_mut<F, R>(&self, f: F) -> R where F: FnOnce(&Self::MutableAccess) -> R {
        let borrowed_mut = self.borrow_mut();
        f(unsafe { std::mem::transmute::<&RefMut<T>, &RefMut<'static, T>>(&borrowed_mut) })
    }
}

impl<T: 'static> SharedAccess for Arc<Mutex<T>> {
    type Item = T;
    type ImmutableAccess = MutexGuard<'static, T>;
    type MutableAccess = MutexGuard<'static, T>;

    fn clone_container(&self) -> Self {
        Arc::clone(self)
    }

    fn access<F, R>(&self, f: F) -> R where F: FnOnce(&Self::ImmutableAccess) -> R {
        let lock = self.lock().unwrap();
        let result = f(unsafe { std::mem::transmute::<&MutexGuard<T>, &MutexGuard<'static, T>>(&lock) });
        drop(lock);
        result
    }

    fn access_mut<F, R>(&self, f: F) -> R where F: FnOnce(&Self::MutableAccess) -> R {
        let lock = self.lock().unwrap();
        let result = f(unsafe { std::mem::transmute::<&MutexGuard<T>, &MutexGuard<'static, T>>(&lock) });
        drop(lock);
        result
    }
}

impl<T: 'static> SharedAccess for Arc<RwLock<T>> {
    type Item = T;
    type ImmutableAccess = RwLockReadGuard<'static, T>;
    type MutableAccess = RwLockWriteGuard<'static, T>;

    fn clone_container(&self) -> Self {
        Arc::clone(self)
    }

    fn access<F, R>(&self, f: F) -> R where F: FnOnce(&Self::ImmutableAccess) -> R {
        let lock = self.read().unwrap();
        let result = f(unsafe { std::mem::transmute::<&RwLockReadGuard<T>, &RwLockReadGuard<'static, T>>(&lock) });
        drop(lock);
        result
    }

    fn access_mut<F, R>(&self, f: F) -> R where F: FnOnce(&Self::MutableAccess) -> R {
        let lock = self.write().unwrap();
        let result = f(unsafe { std::mem::transmute::<&RwLockWriteGuard<T>, &RwLockWriteGuard<'static, T>>(&lock) });
        drop(lock);
        result
    }
}
