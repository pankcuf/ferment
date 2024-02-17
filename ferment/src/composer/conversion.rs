use quote::ToTokens;
use std::rc::Rc;
use std::cell::RefCell;
use syn::__private::TokenStream2;
use crate::composer::{ComposerPresenter, ItemComposer};
use crate::composer_impl;
use crate::context::ScopeContext;
use crate::interface::{MapPairPresenter, OwnerIteratorPresenter};
use crate::presentation::{context::OwnedItemPresenterContext, presentable::ScopeContextPresentable};

pub struct ConversionComposer {
    parent: Option<Rc<RefCell<ItemComposer>>>,
    composer: MapPairPresenter,
    context_presenter: ComposerPresenter<ItemComposer, TokenStream2>,
    conversions_presenter: OwnerIteratorPresenter,
    conversion_presenter: MapPairPresenter,
    path: TokenStream2,
    conversions: Vec<TokenStream2>,
}

impl ConversionComposer {
    pub fn new(composer: MapPairPresenter, context_presenter: ComposerPresenter<ItemComposer, TokenStream2>, conversions_presenter: OwnerIteratorPresenter, conversion_presenter: MapPairPresenter, path: TokenStream2, conversions: Vec<TokenStream2>) -> Self {
        Self { parent: None, composer, context_presenter, conversions_presenter, conversion_presenter, path, conversions }
    }
    pub fn add_conversion(&mut self, name: TokenStream2, conversion: TokenStream2) {
        self.conversions.push((self.conversion_presenter)(name, conversion));
    }
}
composer_impl!(ConversionComposer, TokenStream2, |itself: &ConversionComposer, context: &ScopeContext|
    (itself.composer)(
        (itself.context_presenter)(
            &itself.parent.as_ref().unwrap().borrow()),
        (itself.conversions_presenter)(
            (
                itself.path.to_token_stream(),
                itself.conversions
                .iter()
                .map(|c|
                    OwnedItemPresenterContext::Conversion(c.clone()))
                .collect::<Vec<_>>()))
        .present(context)));

