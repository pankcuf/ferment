use std::rc::Rc;
use std::cell::RefCell;
use syn::__private::TokenStream2;
use crate::composer::ComposerPresenter;
use crate::composer::item::ItemComposer;
use crate::composer_impl;
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::interface::{OwnerIteratorPresenter, ScopeTreeFieldTypedPresenter};
use crate::presentation::{context::OwnedItemPresenterContext, presentable::ScopeContextPresentable};

pub struct FieldsComposer {
    parent: Option<Rc<RefCell<ItemComposer>>>,
    context_presenter: ComposerPresenter<ItemComposer, TokenStream2>,
    pub root_presenter: OwnerIteratorPresenter,
    pub field_presenter: ScopeTreeFieldTypedPresenter,

    pub fields: Vec<OwnedItemPresenterContext>,
}

impl FieldsComposer {
    pub const fn new(root_presenter: OwnerIteratorPresenter, context_presenter: ComposerPresenter<ItemComposer, TokenStream2>, field_presenter: ScopeTreeFieldTypedPresenter, fields: Vec<OwnedItemPresenterContext>) -> Self {
        Self { parent: None, root_presenter, context_presenter, field_presenter, fields }
    }

    pub fn add_conversion(&mut self, field_type: FieldTypeConversion) {
        let value = (self.field_presenter)(field_type);
        self.fields.push(value);
    }
}

composer_impl!(FieldsComposer, TokenStream2, |itself: &FieldsComposer, context: &ScopeContext|
    (itself.root_presenter)((
        (itself.context_presenter)(
            &itself.parent.as_ref().unwrap().borrow()
        ),
        itself.fields.clone()
    ))
    .present(context));
