use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;
use crate::naming::Name;

pub enum DropInterfacePresentation {
    Empty,
    Full {
        name: Name,
        body: TokenStream2
    }
}

impl ToTokens for DropInterfacePresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Empty =>
                quote!(),
            Self::Full { name, body} =>
                quote!(impl Drop for #name { fn drop(&mut self) { unsafe { #body; } } })
        }.to_tokens(tokens)
    }
}
