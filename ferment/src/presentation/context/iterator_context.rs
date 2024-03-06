use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma, Paren};
use crate::context::ScopeContext;
use crate::presentation::context::owned_item_presenter_context::OwnedItemPresentableContext;
use crate::presentation::ScopeContextPresentable;
use crate::wrapped::Wrapped;

pub enum IteratorPresentationContext {
    Empty,
    Curly(Punctuated<OwnedItemPresentableContext, Comma>),
    Round(Punctuated<OwnedItemPresentableContext, Comma>),
}

impl ScopeContextPresentable for IteratorPresentationContext {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            IteratorPresentationContext::Empty => quote!(),
            IteratorPresentationContext::Curly(items) =>
                Wrapped::<_, Brace>::new(items.present(source))
                    .to_token_stream(),
            IteratorPresentationContext::Round(items) =>
                Wrapped::<_, Paren>::new(items.present(source))
                    .to_token_stream(),
        }
    }
}
