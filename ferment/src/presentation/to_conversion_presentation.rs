use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::naming::DictionaryExpression;
#[derive(Clone, Debug)]
pub enum ToConversionPresentation {
    Enum(Punctuated<TokenStream2, Comma>),
    Struct(TokenStream2),
    Tuple(Punctuated<TokenStream2, Comma>),
    Map(TokenStream2, TokenStream2),
    Result(TokenStream2, TokenStream2)
}

impl ToTokens for ToConversionPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            ToConversionPresentation::Enum(conversions) => {
                DictionaryExpression::BoxedExpression(quote!(match obj {
                    #conversions,
                    _ => unreachable!("Enum Variant unreachable")

                }))
                    .to_token_stream()
            },
            ToConversionPresentation::Struct(conversion) => {
                quote! { #conversion }
            },
            ToConversionPresentation::Map(to_key_conversion, to_value_conversion) =>
                quote!(ferment_interfaces::boxed(Self { count: obj.len(), keys: #to_key_conversion, values: #to_value_conversion  })),
            ToConversionPresentation::Result(to_ok_conversion, to_error_conversion) => {
                DictionaryExpression::BoxedExpression(quote!({
                    let (ok, error) = match obj {
                        Ok(o) => (#to_ok_conversion, std::ptr::null_mut()),
                        Err(o) => (std::ptr::null_mut(), #to_error_conversion)
                    };
                    Self { ok, error }
                })).to_token_stream()
            },
            ToConversionPresentation::Tuple(conversions) => quote! {
                ferment_interfaces::boxed(Self { #conversions })
            }
        }.to_tokens(tokens)
    }
}
