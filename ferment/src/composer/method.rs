use quote::ToTokens;
use crate::composer::{Composer, BindingComposer, LocalConversionContext, BindingAccessorContext, DestructorContext, SharedComposer};
use crate::context::ScopeContext;
use crate::ext::FFIResolveExtended;
use crate::presentation::{BindingPresentation, ScopeContextPresentable};
use crate::shared::{HasParent, SharedAccess};

pub struct MethodComposer<Parent: SharedAccess, BCTX: Clone, CTX: Clone> {
    parent: Option<Parent>,
    context_composer: SharedComposer<Parent, CTX>,
    binding_presenter: BindingComposer<BCTX>,
}
impl<Parent: SharedAccess, BCTX: Clone, CTX: Clone> MethodComposer<Parent, BCTX, CTX> {
    pub const fn new(
        binding_presenter: BindingComposer<BCTX>,
        context_composer: SharedComposer<Parent, CTX>) -> Self {
        Self {
            parent: None,
            binding_presenter,
            context_composer,
        }
    }
}
impl<Parent, BCTX, CTX> HasParent<Parent> for MethodComposer<Parent, BCTX, CTX>
    where Parent: SharedAccess,
          BCTX: Clone,
          CTX: Clone {
    fn set_parent(&mut self, parent: &Parent) {
        self.parent = Some(parent.clone_container());
    }
}

impl<Parent: SharedAccess> Composer<Parent> for MethodComposer<Parent, BindingAccessorContext, LocalConversionContext> {
    type Source = ScopeContext;
    type Result = Vec<BindingPresentation>;

    fn compose(&self, source: &Self::Source) -> Self::Result {
        let parent = self.parent.as_ref().unwrap();
        let context = parent.access(self.context_composer);
        let result = context.1.iter()
            .map(|field_type| (self.binding_presenter)((
                context.0.present(source),
                field_type.name(),
                field_type.ty()
                    .ffi_full_dictionary_type_presenter(source)
                    .to_token_stream())))
            .collect();

        result
    }
}

impl<Parent: SharedAccess> Composer<Parent> for MethodComposer<Parent, DestructorContext, DestructorContext> {
    type Source = ScopeContext;
    type Result = BindingPresentation;

    fn compose(&self, _source: &Self::Source) -> Self::Result {
        let parent = self.parent.as_ref().unwrap();
        let context = parent.access(self.context_composer);
        (self.binding_presenter)(context.clone())
    }
}
