use quote::ToTokens;
use crate::composer::{Composer, SharedComposer, BindingComposer, LocalConversionContext, BindingAccessorContext, DestructorContext};
use crate::context::ScopeContext;
use crate::presentation::BindingPresentation;
use crate::shared::{HasParent, SharedAccess};

// #[derive(Parent)]
pub struct MethodComposer<Parent: SharedAccess, BCTX: Clone, CTX: Clone> {
    parent: Option<Parent>,
    // root_composer: ComposerPresenter<LocalConversionContext, Vec<BindingPresentation>>,
    context_composer: SharedComposer<Parent, CTX>,
    binding_presenter: BindingComposer<BCTX>,
}
impl<Parent: SharedAccess, BCTX: Clone, CTX: Clone> MethodComposer<Parent, BCTX, CTX> {
    pub const fn new(
        // root_composer: ComposerPresenter<LocalConversionContext, Vec<BindingPresentation>>,
        binding_presenter: BindingComposer<BCTX>,
        context_composer: SharedComposer<Parent, CTX>) -> Self {
        Self {
            parent: None,
            // root_composer,
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
                context.0.clone(),
                field_type.name(),
                source.ffi_full_dictionary_field_type_presenter(field_type.ty())
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
        (self.binding_presenter)(context)
    }
}

// impl<Parent: SharedAccess> Composer<Parent> for MethodComposer<Parent, EnumVariantConstructorContext, EnumVariantConstructorContext> {
//     type Source = ScopeContext;
//     type Result = BindingPresentation;
//
//     fn compose(&self, _source: &Self::Source) -> Self::Result {
//         let parent = self.parent.as_ref().unwrap();
//         let context = parent.access(self.context_composer);
//         (self.binding_presenter)(context)
//     }
// }

// impl<Parent: SharedAccess> Composer<Parent> for MethodComposer<Parent, StructConstructorContext, StructConstructorContext> {
//     type Source = ScopeContext;
//     type Result = BindingPresentation;
//
//     fn compose(&self, _source: &Self::Source) -> Self::Result {
//         let parent = self.parent.as_ref().unwrap();
//         let context = parent.access(self.context_composer);
//         (self.binding_presenter)(context)
//     }
// }
