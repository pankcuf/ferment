use crate::composer::{Composer, GenericComposerInfo, SliceComposer};
use crate::context::ScopeContext;
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};

impl<'a, SPEC> Composer<'a> for SliceComposer<ObjCFermentate, SPEC>
    where SPEC: ObjCSpecification {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<ObjCFermentate, SPEC>>;

    #[allow(unused_variables)]
    fn compose(&self, source: &'a Self::Source) -> Self::Output {
        None
    }
}