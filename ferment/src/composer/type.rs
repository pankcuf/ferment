use crate::composer::r#abstract::Linkable;
use crate::presentable::{Aspect, ScopeContextPresentable};
use crate::shared::SharedAccess;

#[allow(dead_code)]
pub enum FFIAspect {
    Target,
    FFI,
}

pub struct TypeComposer<Parent, CTX> where Parent: SharedAccess, CTX: Clone, Aspect<CTX>: ScopeContextPresentable {
    pub parent: Option<Parent>,
    pub context: CTX,
}

impl<Parent, CTX> Linkable<Parent> for TypeComposer<Parent, CTX> where Parent: SharedAccess, CTX: Clone, Aspect<CTX>: ScopeContextPresentable {
    fn link(&mut self, parent: &Parent) {
        self.parent = Some(parent.clone_container());
    }
}

impl<Parent, CTX> TypeComposer<Parent, CTX> where Parent: SharedAccess, CTX: Clone, Aspect<CTX>: ScopeContextPresentable {
    pub const fn new(context: CTX) -> Self {
        Self { context, parent: None }
    }

    // pub fn compose_aspect(&self, aspect: FFIAspect) -> Aspect {
    //     match aspect {
    //         FFIAspect::FFI => Aspect::FFI(&self.context),
    //         FFIAspect::Target => Aspect::Target(&self.context),
    //     }
    // }
}
