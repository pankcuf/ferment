use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use crate::composer::{BraceWrapped, CommaPunctuated, ParenWrapped};
use crate::context::ScopeContext;
use crate::presentation::context::owned_item_presenter_context::OwnedItemPresentableContext;
use crate::presentation::ScopeContextPresentable;

pub enum IteratorPresentationContext {
    Empty,
    Curly(CommaPunctuated<OwnedItemPresentableContext>),
    Round(CommaPunctuated<OwnedItemPresentableContext>),
}

impl ScopeContextPresentable for IteratorPresentationContext {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            IteratorPresentationContext::Empty => quote!(),
            IteratorPresentationContext::Curly(items) =>
                BraceWrapped::new(items.present(source))
                    .to_token_stream(),
            IteratorPresentationContext::Round(items) =>
                ParenWrapped::new(items.present(source))
                    .to_token_stream(),
        }
    }
}
