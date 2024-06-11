use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;
use syn::Type;

#[derive(Clone, Debug)]
pub enum DropInterfacePresentation {
    Empty,
    Full {
        attrs: TokenStream2,
        ty: Type,
        body: TokenStream2
    }
}

impl ToTokens for DropInterfacePresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Full { attrs, ty, body} => quote! {
                #attrs
                impl Drop for #ty { fn drop(&mut self) { unsafe { #body; } } }
            },
            Self::Empty => quote!(),
        }.to_tokens(tokens)
    }
}
