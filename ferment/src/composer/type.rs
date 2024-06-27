use crate::composer::r#abstract::Linkable;
use crate::presentable::Context;
use crate::shared::SharedAccess;

#[allow(dead_code)]
pub enum FFIAspect {
    Target,
    FFI,
}

pub struct TypeComposer<Parent> where Parent: SharedAccess {
    pub parent: Option<Parent>,
    pub context: Context,
}

impl<Parent> Linkable<Parent> for TypeComposer<Parent> where Parent: SharedAccess {
    fn link(&mut self, parent: &Parent) {
        self.parent = Some(parent.clone_container());
    }
}

impl<Parent> TypeComposer<Parent> where Parent: SharedAccess {
    pub const fn new(context: Context) -> Self {
        Self { context, parent: None }
    }

    // pub fn compose_aspect(&self, aspect: FFIAspect) -> Aspect {
    //     match aspect {
    //         FFIAspect::FFI => Aspect::FFI(&self.context),
    //         FFIAspect::Target => Aspect::Target(&self.context),
    //     }
    // }
}
