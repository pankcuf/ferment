use crate::composer::r#abstract::Linkable;
use crate::presentable::NameTreeContext;
use crate::shared::SharedAccess;

#[allow(dead_code)]
pub enum FFIAspect {
    Target,
    FFI,
}

pub struct TypeComposer<Link, TYC>
    where Link: SharedAccess,
          TYC: NameTreeContext {
    parent: Option<Link>,
    pub context: TYC,
}

impl<Link, TYC> Linkable<Link> for TypeComposer<Link, TYC>
    where Link: SharedAccess,
          TYC: NameTreeContext {
    fn link(&mut self, parent: &Link) {
        self.parent = Some(parent.clone_container());
    }
}

impl<Link, TYC> TypeComposer<Link, TYC>
    where Link: SharedAccess,
          TYC: NameTreeContext {
    pub const fn new(context: TYC) -> Self {
        Self { context, parent: None }
    }
}
