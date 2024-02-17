use std::cell::RefCell;
use std::rc::Rc;
use quote::ToTokens;
use syn::__private::TokenStream2;
use crate::composer::{Composer, ComposerPresenter, ItemComposer};
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::presentation::BindingPresentation;

pub struct MethodComposer {
    parent: Option<Rc<RefCell<ItemComposer>>>,
    root_type_composer: ComposerPresenter<ItemComposer, TokenStream2>,
    // root_presenter: ComposerPresenter<ItemComposer, Vec<BindingPresentation>>,
    pub binding_presenter: ComposerPresenter<(TokenStream2, TokenStream2, TokenStream2), BindingPresentation>,
    pub fields: Vec<FieldTypeConversion>,
}
impl MethodComposer {
    pub const fn new(
        // root_presenter: ComposerPresenter<ItemComposer, Vec<BindingPresentation>>,
        binding_presenter: ComposerPresenter<(TokenStream2, TokenStream2, TokenStream2), BindingPresentation>,
        root_type_composer: ComposerPresenter<ItemComposer, TokenStream2>) -> Self {
        Self { parent: None, binding_presenter, root_type_composer, fields: vec![] }
    }

    pub fn add_conversion(&mut self, field_type: FieldTypeConversion) {
        self.fields.push(field_type);
    }
}

impl Composer for MethodComposer {
    type Item = Vec<BindingPresentation>;
    fn set_parent(&mut self, root: &Rc<RefCell<ItemComposer>>) {
        self.parent = Some(Rc::clone(root));
    }
    #[allow(clippy::redundant_closure_call)]
    fn compose(&self, context: &ScopeContext) -> Self::Item {
        let root_obj_type = (self.root_type_composer)(&self.parent.as_ref().unwrap().borrow());
        self.fields.iter()
            .map(|field_type| (self.binding_presenter)(
                &(root_obj_type.clone(),
                  field_type.name(),
                  context.ffi_full_dictionary_field_type_presenter(field_type.ty())
                      .to_token_stream())))
            .collect()
    }
}
