use crate::composer::{Composer, NameComposer};
use crate::context::ScopeContext;
use crate::naming::Name;
use crate::shared::{HasParent, SharedAccess};

#[allow(dead_code)]
pub enum FFIAspect {
    Target,
    FFI,
}

pub struct TypeComposer<Parent> where Parent: SharedAccess {
    pub parent: Option<Parent>,
    pub target_name_composer: NameComposer<Parent>,
    pub ffi_name_composer: NameComposer<Parent>,
}

impl<Parent> HasParent<Parent> for TypeComposer<Parent> where Parent: SharedAccess {
    fn set_parent(&mut self, parent: &Parent) {
        self.target_name_composer.set_parent(parent);
        self.ffi_name_composer.set_parent(parent);
        self.parent = Some(parent.clone_container());
    }
}

impl<Parent> TypeComposer<Parent> where Parent: SharedAccess {
    pub const fn new(
        ffi_name_composer: NameComposer<Parent>,
        target_name_composer: NameComposer<Parent>,
    ) -> Self {
        Self { ffi_name_composer, target_name_composer, parent: None }
    }

    pub fn compose_aspect(&self, aspect: FFIAspect, source: &ScopeContext) -> Name {
        match aspect {
            FFIAspect::FFI =>
                self.ffi_name_composer
                    .compose(source),
            FFIAspect::Target =>
                self.target_name_composer
                    .compose(source),
        }
    }
}
