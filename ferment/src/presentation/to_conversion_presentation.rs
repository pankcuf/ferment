use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::naming::DictionaryFieldName;

pub enum ToConversionPresentation {
    Enum(Punctuated<TokenStream2, Comma>),
    Struct(TokenStream2),
    Map(TokenStream2, TokenStream2),
    Result(TokenStream2, TokenStream2)
}

impl ToTokens for ToConversionPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            ToConversionPresentation::Enum(conversions) => {
                DictionaryFieldName::BoxedExpression(quote!(match obj { #conversions }))
                    .to_token_stream()
            },
            ToConversionPresentation::Struct(conversion) => {
                quote! { #conversion }
            },
            ToConversionPresentation::Map(to_key_conversion, to_value_conversion) =>
                quote!(ferment_interfaces::boxed(Self { count: obj.len(), keys: #to_key_conversion, values: #to_value_conversion  })),
            ToConversionPresentation::Result(to_ok_conversion, to_error_conversion) => quote! {
                let (ok, error) = match obj {
                    Ok(o) => (#to_ok_conversion, std::ptr::null_mut()),
                    Err(o) => (std::ptr::null_mut(), #to_error_conversion)
                };
                ferment_interfaces::boxed(Self { ok, error })
            }
        }.to_tokens(tokens)
    }
}
