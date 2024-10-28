use crate::composer::{CallbackComposer, SourceComposable, GenericComposerInfo};
use crate::context::ScopeContext;
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};
use crate::presentable::{ArgKind, ScopeContextPresentable, SeqKind};

impl<SPEC> SourceComposable for CallbackComposer<ObjCFermentate, SPEC>
    where SPEC: ObjCSpecification,
          SeqKind<ObjCFermentate, SPEC>: ScopeContextPresentable,
          ArgKind<ObjCFermentate, SPEC>: ScopeContextPresentable {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<ObjCFermentate, SPEC>>;

    fn compose(&self, _source: &Self::Source) -> Self::Output {
        None
    }
}