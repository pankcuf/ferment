use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;

pub enum DropInterfacePresentation {
    Empty,
    Full(TokenStream2, TokenStream2)
}

impl ToTokens for DropInterfacePresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Empty => quote!(),
            Self::Full(name, code) =>
                quote!(impl Drop for #name { fn drop(&mut self) { unsafe { #code } } })
        }.to_tokens(tokens)
    }
}
