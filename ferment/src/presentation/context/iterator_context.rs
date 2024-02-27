use quote::quote;
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::token::{Comma, Semi};
use crate::context::ScopeContext;
use crate::presentation::context::owned_item_presenter_context::OwnedItemPresentableContext;
use crate::presentation::context::{FieldTypePresentableContext, OwnerIteratorPresentationContext};
use crate::presentation::ScopeContextPresentable;

pub enum IteratorPresentationContext {
    Empty,
    DropCode(Punctuated<OwnedItemPresentableContext, Semi>),
    Curly(Punctuated<OwnedItemPresentableContext, Comma>),
    Round(Punctuated<OwnedItemPresentableContext, Comma>),
    StructDropBody(Punctuated<OwnedItemPresentableContext, Semi>),
    EnumDropBody(Punctuated<OwnedItemPresentableContext, Comma>),
    Lambda(Box<OwnerIteratorPresentationContext>, Box<IteratorPresentationContext>),
}

impl ScopeContextPresentable for IteratorPresentationContext {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            IteratorPresentationContext::Empty => quote!(),
            IteratorPresentationContext::DropCode(items) => {
                let items = items.present(source);
                quote!({ #items })
            },
            IteratorPresentationContext::Curly(items) => {
                let items = items.present(source);
                quote!({ #items })
            },
            IteratorPresentationContext::Round(items) => {
                let items = items.present(source);
                quote!(( #items ))
            },
            IteratorPresentationContext::StructDropBody(items) => {
                match items.len() {
                    0 => quote!(),
                    _ => {
                        let items = items.present(source);
                        quote!(let ffi_ref = self; #items)
                    }
                }
            },
            IteratorPresentationContext::EnumDropBody(items) => {
                match items.len() {
                    0 => quote!(),
                    _ => OwnerIteratorPresentationContext::MatchFields((FieldTypePresentableContext::Simple(quote!(self)).into(), items.clone()))
                        .present(source)
                }
            }
            IteratorPresentationContext::Lambda(l_value, r_value) => {
                let l_value = l_value.present(source);
                let r_value = r_value.present(source);
                quote!(#l_value => #r_value)
            }
        }
    }
}
