use quote::ToTokens;
use syn::__private::TokenStream2;
use crate::ext::Terminated;
use crate::interface::package_unboxed_root;

pub enum DestroyPresentation {
    Default,
    Custom(TokenStream2)
}

impl ToTokens for DestroyPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Default =>
                package_unboxed_root()
                    .terminated()
                    .to_tokens(tokens),
            DestroyPresentation::Custom(conversion) =>
                conversion.to_tokens(tokens)
        }
    }
}