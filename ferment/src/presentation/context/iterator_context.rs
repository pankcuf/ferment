use quote::quote;
use syn::__private::TokenStream2;
use crate::context::ScopeContext;
use crate::presentation::context::owned_item_presenter_context::OwnedItemPresenterContext;
use crate::presentation::context::OwnerIteratorPresentationContext;
use crate::presentation::ScopeContextPresentable;

pub enum IteratorPresentationContext {
    Empty,
    DefaultDestroyFields(Vec<OwnedItemPresenterContext>),
    Curly(Vec<OwnedItemPresenterContext>),
    Round(Vec<OwnedItemPresenterContext>),
    StructDestroy(Vec<OwnedItemPresenterContext>),
    EnumDestroy(Vec<OwnedItemPresenterContext>),
}

impl ScopeContextPresentable for IteratorPresentationContext {
    type Presentation = TokenStream2;

    fn present(&self, context: &ScopeContext) -> Self::Presentation {
        match self {
            IteratorPresentationContext::Empty => quote!(),
            IteratorPresentationContext::DefaultDestroyFields(items) => {
                let items = items.iter().map(|f| f.present(context));
                quote!({ #(#items;)* })
            },
            IteratorPresentationContext::Curly(items) => {
                let items = items.iter().map(|f| f.present(context));
                quote!({ #(#items,)* })
            },
            IteratorPresentationContext::Round(items) => {
                let items = items.iter().map(|f| f.present(context));
                quote!(( #(#items,)* ))
            },
            IteratorPresentationContext::StructDestroy(items) => {
                match items.len() {
                    0 => quote!(),
                    _ => {
                        let items = items.iter().map(|f| f.present(context));
                        quote!(let ffi_ref = self; #(#items;)*)
                    }
                }
            },
            IteratorPresentationContext::EnumDestroy(items) => {
                match items.len() {
                    0 => quote!(),
                    _ => OwnerIteratorPresentationContext::MatchFields((quote!(self), items.clone()))
                        .present(context)
                }
            }
        }
    }
}
