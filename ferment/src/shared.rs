use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;

pub trait SharedAccess {
    type Item;
    type ImmutableAccess;
    type MutableAccess;

    fn clone_container(&self) -> Self where Self: Sized;

    fn access<F, R>(&self, f: F) -> R
        where
            F: FnOnce(&Self::ImmutableAccess) -> R;

    #[allow(unused)]
    fn access_mut<F, R>(&self, f: F) -> R
        where
            F: FnOnce(&Self::MutableAccess) -> R;
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
