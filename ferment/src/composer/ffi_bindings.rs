use syn::__private::TokenStream2;
use ferment_macro::Parent;
use crate::composer::{Composer, FieldTypesContext, IteratorConversionComposer, OwnedFieldTypeComposer};
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::presentation::context::OwnedItemPresenterContext;
use crate::presentation::ScopeContextPresentable;
use crate::shared::SharedAccess;

#[derive(Parent)]
pub struct FFIBindingsComposer<Parent: SharedAccess> {
    parent: Option<Parent>,
    root_presenter: IteratorConversionComposer,
    field_types: FieldTypesContext,
    sig_argument_presenter: OwnedFieldTypeComposer,
    field_names_presenter: OwnedFieldTypeComposer,
}

impl<Parent: SharedAccess> Composer<Parent> for FFIBindingsComposer<Parent> {
    type Item = TokenStream2;
    type Source = ScopeContext;

    fn compose(&self, source: &Self::Source) -> Self::Item {
        (self.root_presenter)(self.field_types.iter().map(|ff| OwnedItemPresenterContext::DefaultField(ff.clone())).collect::<Vec<_>>()).present(source)
    }
}

impl<Parent: SharedAccess> FFIBindingsComposer<Parent> {
    pub const fn new(
        root_presenter: IteratorConversionComposer,
        sig_argument_presenter: OwnedFieldTypeComposer,
        field_names_presenter: OwnedFieldTypeComposer
    ) -> Self {
        Self { parent: None, root_presenter, sig_argument_presenter, field_names_presenter, field_types: vec![] }
    }
    pub(crate) fn add_conversion(&mut self, field_type: FieldTypeConversion) {
        self.field_types.push(field_type);
    }

    fn compose_with_item_presenter(&self, item_presenter: OwnedFieldTypeComposer) -> Vec<OwnedItemPresenterContext> {
        self.field_types.iter()
            .map(|field_type| (item_presenter)(field_type.clone()))
            .collect()
    }

    pub fn present_field_names(&self, context: &ScopeContext) -> TokenStream2 {
        (self.root_presenter)(self.compose_with_item_presenter(self.field_names_presenter))
            .present(context)
    }

    pub fn compose_arguments(&self) -> Vec<OwnedItemPresenterContext> {
        self.compose_with_item_presenter(self.sig_argument_presenter)
    }
}

