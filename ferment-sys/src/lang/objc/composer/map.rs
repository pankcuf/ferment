use crate::composer::{Composer, GenericComposerInfo, MapComposer};
use crate::context::ScopeContext;
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};

impl<'a, SPEC> Composer<'a> for MapComposer<ObjCFermentate, SPEC>
    where SPEC: ObjCSpecification {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<ObjCFermentate, SPEC>>;

    #[allow(unused_variables)]
    fn compose(&self, source: &'a Self::Source) -> Self::Output {
        None
    }
}