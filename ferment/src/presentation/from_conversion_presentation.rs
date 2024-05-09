use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;
use syn::punctuated::Punctuated;
use syn::token::Comma;

#[derive(Clone, Debug)]
pub enum FromConversionPresentation {
    Just(TokenStream2),
    Tuple(Punctuated<TokenStream2, Comma>),
    Enum(Punctuated<TokenStream2, Comma>),
    Map(TokenStream2, TokenStream2),
    Result(TokenStream2, TokenStream2),
}

impl ToTokens for FromConversionPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            FromConversionPresentation::Just(conversion) => {
                quote!(#conversion)
            },
            FromConversionPresentation::Enum(conversions) => quote! {
                let ffi_ref = &*ffi;
                match ffi_ref {
                    #conversions
                }
            },
            FromConversionPresentation::Map(from_key_conversion, from_value_conversion) => quote! {
                let ffi_ref = &*ffi;
                ferment_interfaces::fold_to_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values, #from_key_conversion, #from_value_conversion)
            },
            FromConversionPresentation::Result(from_ok_conversion, from_error_conversion) => quote! {
                let ffi_ref = &*ffi;
                ferment_interfaces::fold_to_result(ffi_ref.ok, ffi_ref.error, #from_ok_conversion, #from_error_conversion)
            },
            FromConversionPresentation::Tuple(conversions) => quote! {
                let ffi_ref = &*ffi;
                (#conversions)
            }
        }.to_tokens(tokens)
    }
}
