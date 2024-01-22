use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;

pub enum FromConversionPresentation {
    Enum(Vec<TokenStream2>),
    Struct(TokenStream2),
    Vec,
    Map(TokenStream2, TokenStream2),
    Result(TokenStream2, TokenStream2),
}

impl ToTokens for FromConversionPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            FromConversionPresentation::Enum(conversions) => {
                quote! {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        #(#conversions,)*
                    }
                }
            },
            FromConversionPresentation::Struct(conversion) => {
                quote! {
                    #conversion
                }
            },
            FromConversionPresentation::Vec =>
                quote!(ferment_interfaces::FFIVecConversion::decode(&*ffi)),
            FromConversionPresentation::Map(from_key_conversion, from_value_conversion) => quote! {
                let ffi_ref = &*ffi;
                ferment_interfaces::fold_to_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values, #from_key_conversion, #from_value_conversion)
            },
            FromConversionPresentation::Result(from_ok_conversion, from_error_conversion) => quote! {
                let ffi_ref = &*ffi;
                ferment_interfaces::fold_to_result(ffi_ref.ok, ffi_ref.error, #from_ok_conversion, #from_error_conversion)
            }
        }.to_tokens(tokens)
    }
}
