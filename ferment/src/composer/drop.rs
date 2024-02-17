use std::rc::Rc;
use std::cell::RefCell;
use syn::__private::TokenStream2;
use crate::composer::{ComposerPresenter, ItemComposer};
use crate::composer_impl;
use crate::context::ScopeContext;
use crate::interface::{IteratorPresenter, MapPairPresenter, MapPresenter};
use crate::presentation::{context::OwnedItemPresenterContext, presentable::ScopeContextPresentable};
pub struct DropComposer {
    parent: Option<Rc<RefCell<ItemComposer>>>,
    composer: MapPairPresenter,
    context_presenter: ComposerPresenter<ItemComposer, TokenStream2>,
    conversions_presenter: IteratorPresenter,
    conversion_presenter: MapPresenter,
    conversions: Vec<TokenStream2>,
}

impl DropComposer {
    pub const fn new(composer: MapPairPresenter, context_presenter: ComposerPresenter<ItemComposer, TokenStream2>, conversions_presenter: IteratorPresenter, conversion_presenter: MapPresenter, conversions: Vec<TokenStream2>) -> Self {
        Self { parent: None, composer, context_presenter, conversions_presenter, conversion_presenter, conversions }
    }

    pub fn add_conversion(&mut self, conversion: &TokenStream2) {
        let value = (self.conversion_presenter)(conversion);
        self.conversions.push(value);
    }
}
composer_impl!(DropComposer, TokenStream2, |itself: &DropComposer, context: &ScopeContext|
    (itself.composer)(
        (itself.context_presenter)(
            &itself.parent.as_ref().unwrap().borrow()),
        (itself.conversions_presenter)(
            itself.conversions
            .iter()
            .map(|c|
                OwnedItemPresenterContext::Conversion(c.clone()))
            .collect())
        .present(context)));

