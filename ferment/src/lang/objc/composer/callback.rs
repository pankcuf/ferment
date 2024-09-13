use syn::Generics;
use crate::composer::{CallbackComposer, Composer};
use crate::context::ScopeContext;
use crate::lang::objc::composers::AttrWrapper;
use crate::lang::objc::ObjCFermentate;

impl<'a> Composer<'a> for CallbackComposer<ObjCFermentate, AttrWrapper, Option<Generics>> {
    type Source = ScopeContext;
    type Output = ObjCFermentate;

    fn compose(&self, _source: &'a Self::Source) -> Self::Output {
        todo!()
    }
}