use syn::__private::TokenStream2;
use ferment_macro::Parent;
use crate::composer::{Composer, FieldTypesContext, IteratorConversionComposer, OwnedFieldTypeComposer, SharedComposer};
use crate::context::ScopeContext;
use crate::presentation::context::{IteratorPresentationContext, OwnedItemPresenterContext};
use crate::presentation::ScopeContextPresentable;
use crate::shared::SharedAccess;

#[derive(Parent)]
pub struct FFIBindingsComposer<Parent: SharedAccess> {
    parent: Option<Parent>,
    root_composer: IteratorConversionComposer,
    context_composer: SharedComposer<Parent, FieldTypesContext>,

    sig_argument_presenter: OwnedFieldTypeComposer,
    field_names_presenter: OwnedFieldTypeComposer,
}

impl<Parent: SharedAccess> Composer<Parent> for FFIBindingsComposer<Parent> {
    type Item = TokenStream2;
    type Source = ScopeContext;

    fn compose(&self, source: &Self::Source) -> Self::Item {
        let parent = self.parent.as_ref().unwrap();
        let context = parent.access(self.context_composer);
        let context = context.iter().map(|ff| OwnedItemPresenterContext::DefaultField(ff.clone())).collect::<Vec<_>>();
        (self.root_composer)(context).present(source)
    }
}

impl<Parent: SharedAccess> FFIBindingsComposer<Parent> {
    pub const fn new(
        root_composer: IteratorConversionComposer,
        context_composer: SharedComposer<Parent, FieldTypesContext>,
        sig_argument_presenter: OwnedFieldTypeComposer,
        field_names_presenter: OwnedFieldTypeComposer
    ) -> Self {
        Self { parent: None, root_composer, context_composer, sig_argument_presenter, field_names_presenter }
    }

    fn compose_with_item_presenter(&self, item_presenter: OwnedFieldTypeComposer) -> Vec<OwnedItemPresenterContext> {
        let parent = self.parent.as_ref().unwrap();
        let context = parent.access(self.context_composer);
        context.iter()
            .map(|field_type| (item_presenter)(field_type.clone()))
            .collect()
    }

    pub fn compose_field_names(&self) -> IteratorPresentationContext {
        (self.root_composer)(self.compose_with_item_presenter(self.field_names_presenter))
    }

    pub fn compose_arguments(&self) -> Vec<OwnedItemPresenterContext> {
        self.compose_with_item_presenter(self.sig_argument_presenter)
    }
}

