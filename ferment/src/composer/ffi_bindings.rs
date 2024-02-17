use std::rc::Rc;
use std::cell::RefCell;
use syn::__private::TokenStream2;
use crate::composer::{Composer, ItemComposer};
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::interface::{IteratorPresenter, ScopeTreeFieldTypedPresenter};
use crate::presentation::context::OwnedItemPresenterContext;
use crate::presentation::ScopeContextPresentable;

pub struct FFIBindingsComposer {
    parent: Option<Rc<RefCell<ItemComposer>>>,
    root_presenter: IteratorPresenter,
    field_types: Vec<FieldTypeConversion>,
    sig_argument_presenter: ScopeTreeFieldTypedPresenter,
    field_names_presenter: ScopeTreeFieldTypedPresenter,
}
impl Composer for FFIBindingsComposer {
    type Item = TokenStream2;

    fn set_parent(&mut self, root: &Rc<RefCell<ItemComposer>>) {
        self.parent = Some(Rc::clone(root));
    }

    fn compose(&self, context: &ScopeContext) -> Self::Item {
        (self.root_presenter)(self.field_types.iter().map(|ff| OwnedItemPresenterContext::DefaultField(ff.clone())).collect::<Vec<_>>()).present(context)
    }
}

impl FFIBindingsComposer {
    pub const fn new(root_presenter: IteratorPresenter, sig_argument_presenter: ScopeTreeFieldTypedPresenter, field_names_presenter: ScopeTreeFieldTypedPresenter) -> Self {
        Self { parent: None, root_presenter, sig_argument_presenter, field_names_presenter, field_types: vec![] }
    }
    pub(crate) fn add_conversion(&mut self, field_type: FieldTypeConversion) {
        self.field_types.push(field_type);
    }

    fn compose_with_item_presenter(&self, item_presenter: ScopeTreeFieldTypedPresenter) -> Vec<OwnedItemPresenterContext> {
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

