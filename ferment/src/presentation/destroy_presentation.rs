use quote::ToTokens;
use syn::__private::TokenStream2;
use crate::ext::Terminated;
use crate::presentation::{DictionaryName, InterfacesMethodExpr};

#[derive(Clone, Debug)]
pub enum DestroyPresentation {
    Default,
    Custom(TokenStream2)
}

impl ToTokens for DestroyPresentation {
    fn to_tokens(&self, dst: &mut TokenStream2) {
        match self {
            Self::Default =>
                InterfacesMethodExpr::UnboxAny(DictionaryName::Ffi.to_token_stream()).to_token_stream().terminated().to_tokens(dst),
            Self::Custom(conversion) =>
                conversion
                    .to_tokens(dst)
        }
    }
}