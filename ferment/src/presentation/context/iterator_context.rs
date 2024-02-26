use quote::quote;
use syn::__private::TokenStream2;
use crate::context::ScopeContext;
use crate::presentation::context::owned_item_presenter_context::OwnedItemPresentableContext;
use crate::presentation::context::{FieldTypePresentableContext, OwnerIteratorPresentationContext};
use crate::presentation::ScopeContextPresentable;

pub enum IteratorPresentationContext {
    Empty,
    // Simple(TokenStream2),
    DefaultDestroyFields(Vec<OwnedItemPresentableContext>),
    Curly(Vec<OwnedItemPresentableContext>),
    Round(Vec<OwnedItemPresentableContext>),
    StructDropBody(Vec<OwnedItemPresentableContext>),
    EnumDropBody(Vec<OwnedItemPresentableContext>),
    Lambda(Box<OwnerIteratorPresentationContext>, Box<IteratorPresentationContext>),
    // Constructor(Vec<OwnedItemPresentableContext>, Vec<OwnedItemPresentableContext>)
}

impl ScopeContextPresentable for IteratorPresentationContext {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            IteratorPresentationContext::Empty => quote!(),
            IteratorPresentationContext::DefaultDestroyFields(items) => {
                let items = items.iter().map(|f| f.present(source));
                quote!({ #(#items;)* })
            },
            IteratorPresentationContext::Curly(items) => {
                let items = items.iter().map(|f| f.present(source));
                quote!({ #(#items,)* })
            },
            IteratorPresentationContext::Round(items) => {
                let items = items.iter().map(|f| f.present(source));
                quote!(( #(#items,)* ))
            },
            IteratorPresentationContext::StructDropBody(items) => {
                match items.len() {
                    0 => quote!(),
                    _ => {
                        let items = items.iter().map(|f| f.present(source));
                        quote!(let ffi_ref = self; #(#items;)*)
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
            // IteratorPresentationContext::Simple(conversion) => {
            //     quote!(#conversion)
            // }
            IteratorPresentationContext::Lambda(l_value, r_value) => {
                let l_value = l_value.present(source);
                let r_value = r_value.present(source);
                quote!(#l_value => #r_value)
            }

            // IteratorPresentationContext::Constructor(_, _) => {}
        }
    }
}
