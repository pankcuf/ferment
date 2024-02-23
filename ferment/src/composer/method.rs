use quote::ToTokens;
use ferment_macro::Parent;
use crate::composer::{Composer, SharedComposer, BindingComposer, LocalConversionContext};
use crate::context::ScopeContext;
use crate::presentation::BindingPresentation;
use crate::shared::SharedAccess;

#[derive(Parent)]
pub struct MethodComposer<Parent: SharedAccess> {
    parent: Option<Parent>,
    // root_composer: ComposerPresenter<LocalConversionContext, Vec<BindingPresentation>>,
    context_composer: SharedComposer<Parent, LocalConversionContext>,
    binding_presenter: BindingComposer,
}
impl<Parent: SharedAccess> MethodComposer<Parent> {
    pub const fn new(
        // root_composer: ComposerPresenter<LocalConversionContext, Vec<BindingPresentation>>,
        binding_presenter: BindingComposer,
        context_composer: SharedComposer<Parent, LocalConversionContext>) -> Self {
        Self {
            parent: None,
            // root_composer,
            binding_presenter,
            context_composer,
        }
    }
}

impl<Parent: SharedAccess> Composer<Parent> for MethodComposer<Parent> {
    type Item = Vec<BindingPresentation>;
    type Source = ScopeContext;

    fn compose(&self, source: &Self::Source) -> Self::Item {
        let parent = self.parent.as_ref().unwrap();
        // let (context, fields) = parent.access(self.context_composer);
        let context = parent.access(self.context_composer);
        // (self.root_composer)(context)
        let result = context.1.iter()
            .map(|field_type| (self.binding_presenter)((context.0.clone(), field_type.name(), source.ffi_full_dictionary_field_type_presenter(field_type.ty()).to_token_stream())))
            .collect();

        result
    }
}
