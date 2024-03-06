use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;
use syn::Type;

pub enum DropInterfacePresentation {
    Empty,
    Full {
        ty: Type,
        body: TokenStream2
    }
}

impl ToTokens for DropInterfacePresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Empty =>
                quote!(),
            Self::Full { ty, body} =>
                quote!(impl Drop for #ty { fn drop(&mut self) { unsafe { #body; } } })
        }.to_tokens(tokens)
    }
}
