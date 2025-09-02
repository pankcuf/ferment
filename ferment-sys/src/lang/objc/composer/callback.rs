use crate::composer::{CallbackComposer, SourceComposable, GenericComposerInfo};
use crate::context::ScopeContext;
use crate::lang::objc::ObjCSpecification;

impl SourceComposable for CallbackComposer<ObjCSpecification> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<ObjCSpecification>>;

    fn compose(&self, _source: &Self::Source) -> Self::Output {
        None
    }
}