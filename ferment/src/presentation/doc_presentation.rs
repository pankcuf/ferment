use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;
use crate::interface::DEFAULT_DOC_PRESENTER;

pub enum DocPresentation {
    Empty,
    Default(TokenStream2),
    Direct(TokenStream2),
    Safety(TokenStream2),
}

impl ToTokens for DocPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Empty => quote!(),
            Self::Direct(target_name) => quote!(#target_name),
            Self::Default(target_name) => DEFAULT_DOC_PRESENTER(&quote!(#target_name)),
            Self::Safety(target_name) => {
                let doc = DEFAULT_DOC_PRESENTER(&quote!(#target_name));
                quote! {
                    #doc
                    /// # Safety
                }
            }
        }.to_tokens(tokens)
    }
}
