use crate::composer::r#abstract::Linkable;
use crate::presentable::{Aspect, NameTreeContext, ScopeContextPresentable};
use crate::shared::SharedAccess;

#[allow(dead_code)]
pub enum FFIAspect {
    Target,
    FFI,
}

pub struct TypeComposer<Link, TYC>
    where Link: SharedAccess,
          TYC: NameTreeContext,
          Aspect<TYC>: ScopeContextPresentable {
    parent: Option<Link>,
    pub context: TYC,
}

impl<Link, TYC> Linkable<Link> for TypeComposer<Link, TYC>
    where Link: SharedAccess,
          TYC: NameTreeContext,
          Aspect<TYC>: ScopeContextPresentable {
    fn link(&mut self, parent: &Link) {
        self.parent = Some(parent.clone_container());
    }
}

impl<Link, TYC> TypeComposer<Link, TYC>
    where Link: SharedAccess,
          TYC: NameTreeContext,
          Aspect<TYC>: ScopeContextPresentable {
    pub const fn new(context: TYC) -> Self {
        Self { context, parent: None }
    }

    // pub fn compose_aspect(&self, aspect: FFIAspect) -> Aspect {
    //     match aspect {
    //         FFIAspect::FFI => Aspect::FFI(&self.context),
    //         FFIAspect::Target => Aspect::Target(&self.context),
    //     }
    // }
}

// impl<'a, Parent, TYC> Composer<'a> for TypeComposer<Parent, TYC>
//     where Parent: SharedAccess,
//           TYC: Clone,
//           Aspect<TYC>: ScopeContextPresentable {
//     type Source = ScopeContext;
//     type Output = TYC;
//
//     fn compose(&self, _source: &'a Self::Source) -> Self::Output {
//         self.context.clone()
//     }
// }
