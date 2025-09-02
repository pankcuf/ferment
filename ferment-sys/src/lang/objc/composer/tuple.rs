use crate::composer::{SourceComposable, GenericComposerInfo, TupleComposer};
use crate::context::ScopeContext;
use crate::lang::objc::ObjCSpecification;

impl SourceComposable for TupleComposer<ObjCSpecification> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<ObjCSpecification>>;

    #[allow(unused_variables)]
    fn compose(&self, source: &Self::Source) -> Self::Output {
        None
    }
}