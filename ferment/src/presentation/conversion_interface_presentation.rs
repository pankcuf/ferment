use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;
use syn::{Generics, Type};
use crate::naming::DictionaryFieldName;
use crate::presentation::{FromConversionPresentation, ToConversionPresentation};
use crate::presentation::destroy_presentation::DestroyPresentation;

#[derive(Clone, Debug)]
pub enum InterfacePresentation {
    Conversion {
        attrs: TokenStream2,
        types: (
            Type, // FFI
            Type // Original
        ),
        conversions: (
            FromConversionPresentation,
            ToConversionPresentation,
            DestroyPresentation,
            Option<Generics>
        ),
    },
    VecConversion {
        attrs: TokenStream2,
        types: (
            Type, // FFI
            Type // Original
        ),
        decode: TokenStream2,
        encode: TokenStream2,
    }
}

impl ToTokens for InterfacePresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            // Self::Empty => quote!(),
            Self::Conversion {
                attrs,
                types: (
                    ffi_type,
                    target_type),
                conversions: (
                    from_presentation,
                    to_presentation,
                    destroy_presentation,
                    generics),
            } => {
                let (generic_bounds, where_clause) = match generics {
                    Some(generics) => {
                        let gens = generics.params.iter().map(|generic_param| generic_param.to_token_stream());
                        let where_clause = match &generics.where_clause {
                            Some(where_clause) => {
                                let where_predicates = where_clause.predicates.iter().map(|predicate| predicate.to_token_stream());
                                quote!(where <#(#where_predicates)*,>)
                            }
                            None => quote!()
                        };
                        let generic_bounds = (gens.len() > 0)
                            .then(|| quote!(<#(#gens)*,>))
                            .unwrap_or_default();
                        (generic_bounds, where_clause)
                    },
                    None => (quote!(), quote!())
                };

                let package = DictionaryFieldName::Package;
                let interface = DictionaryFieldName::Interface;
                let obj = DictionaryFieldName::Obj;
                quote! {
                    #attrs
                    impl #generic_bounds #package::#interface<#target_type> for #ffi_type #where_clause {
                        unsafe fn ffi_from_const(ffi: *const #ffi_type) -> #target_type {
                            #from_presentation
                        }
                        unsafe fn ffi_to_const(#obj: #target_type) -> *const #ffi_type {
                            #to_presentation
                        }
                        unsafe fn destroy(ffi: *mut #ffi_type) {
                            #destroy_presentation;
                        }
                    }
                }
            },
            InterfacePresentation::VecConversion { attrs, types: (ffi_type, target_type), decode, encode } => quote! {
                #attrs
                impl ferment_interfaces::FFIVecConversion for #ffi_type {
                    type Value = #target_type;
                    unsafe fn decode(&self) -> Self::Value { #decode }
                    unsafe fn encode(obj: Self::Value) -> *mut Self { #encode }
                }
            }
        }.to_tokens(tokens)
    }
}
