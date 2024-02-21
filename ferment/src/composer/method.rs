use quote::ToTokens;
use syn::__private::TokenStream2;
use ferment_macro::Parent;
use crate::composer::{Composer, SharedComposer, BindingComposer, FieldTypesContext};
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::presentation::BindingPresentation;
use crate::shared::SharedAccess;

#[derive(Parent)]
pub struct MethodComposer<Parent: SharedAccess> {
    parent: Option<Parent>,
    context_presenter: SharedComposer<Parent, TokenStream2>,
    pub binding_presenter: BindingComposer,
    pub fields: FieldTypesContext,
}
impl<Parent: SharedAccess> MethodComposer<Parent> {
    pub const fn new(
        binding_presenter: BindingComposer,
        context_presenter: SharedComposer<Parent, TokenStream2>) -> Self {
        Self {
            parent: None,
            binding_presenter,
            context_presenter,
            fields: vec![],
        }
    }

    pub fn add_conversion(&mut self, field_type: FieldTypeConversion) {
        self.fields.push(field_type);
    }
}

impl<Parent: SharedAccess> Composer<Parent> for MethodComposer<Parent> {
    type Item = Vec<BindingPresentation>;
    type Source = ScopeContext;

    fn compose(&self, source: &Self::Source) -> Self::Item {
        let parent = self.parent.as_ref().unwrap();
        let context = parent.access(self.context_presenter);

        let result = self.fields.iter()
            .map(|field_type| (self.binding_presenter)((context.clone(), field_type.name(), source.ffi_full_dictionary_field_type_presenter(field_type.ty()).to_token_stream())))
            .collect();

        result
    }
}
