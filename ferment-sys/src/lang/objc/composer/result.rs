use crate::composer::{SourceComposable, GenericComposerInfo, ResultComposer};
use crate::context::ScopeContext;
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};

impl<SPEC> SourceComposable for ResultComposer<ObjCFermentate, SPEC>
    where SPEC: ObjCSpecification {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<ObjCFermentate, SPEC>>;

    #[allow(unused_variables)]
    fn compose(&self, source: &Self::Source) -> Self::Output {


        None
    }
}