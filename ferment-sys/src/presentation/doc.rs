use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;
use syn::parse_quote;
use crate::formatter::format_token_stream;

#[derive(Clone, Debug)]
#[allow(unused)]
pub enum DocPresentation {
    Empty,
    Default(TokenStream2),
    DefaultT(TokenStream2),
    Direct(TokenStream2),
    Safety(TokenStream2),
}

pub fn default_doc<T: ToTokens>(name: T) -> TokenStream2 {
    let comment = format!("FFI-representation of the [`{}`]", format_token_stream(name));
    // TODO: FFI-representation of the [`{}`](../../path/to/{}.rs)
    parse_quote! { #[doc = #comment] }

}

impl ToTokens for DocPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Empty => quote!(),
            Self::Direct(target_name) => quote!(#target_name),
            Self::Default(target_name) => default_doc(target_name),
            Self::DefaultT(target_name) => default_doc(target_name),
            Self::Safety(target_name) => {
                let doc = default_doc(target_name);
                quote! {
                    #doc
                    /// # Safety
                }
            }
        }.to_tokens(tokens)
    }
}
