use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;
use syn::Generics;
use crate::interface::{interface, obj, package};
use crate::presentation::{FromConversionPresentation, ToConversionPresentation};

pub enum ConversionInterfacePresentation {
    Empty,
    Interface {
        ffi_type: TokenStream2,
        target_type: TokenStream2,
        from_presentation: FromConversionPresentation,
        to_presentation: ToConversionPresentation,
        destroy_presentation: TokenStream2,
        generics: Option<Generics>,
    },
    // GenericInterface {
    //     ffi_type: TokenStream2,
    //     target_type: TokenStream2,
    //     from_presentation: FromConversionPresentation,
    //     to_presentation: ToConversionPresentation,
    //     destroy_presentation: TokenStream2,
    //     generics: Vec<TokenStream2>,
    // }
}

impl ToTokens for ConversionInterfacePresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Empty => quote!(),
            Self::Interface {
                ffi_type,
                target_type,
                from_presentation,
                to_presentation,
                destroy_presentation,
                generics} => {
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
                            .then_some(quote!(<#(#gens)*,>))
                            .unwrap_or_default();
                        (generic_bounds, where_clause)
                    },
                    None => (quote!(), quote!())
                };

                let package = package();
                let interface = interface();
                let obj = obj();
                quote! {
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
            // Self::GenericInterface {
            //     ffi_type,
            //     target_type,
            //     from_presentation,
            //     to_presentation,
            //     destroy_presentation,
            //     generics } => {
            //     let package = package();
            //     let interface = interface();
            //     let obj = obj();
            //     let ffi_from_const = ffi_from_const();
            //     let ffi_to_const = ffi_to_const();
            //     //<T: crate::asyn::query::TransportRequest, E: std::error::Error>
            //     //vec![T: crate::asyn::query::TransportRequest, E: std::error::Error]
            //
            //     let generic_bounds = (!generics.is_empty())
            //         .then_some(quote!(<#(#generics)*,>))
            //         .unwrap_or_default();
            //
            //     quote! {
            //         impl #generic_bounds #package::#interface<#target_type> for #ffi_type {
            //             unsafe fn #ffi_from_const(ffi: *const #ffi_type) -> #target_type {
            //                 #from_presentation
            //             }
            //             unsafe fn #ffi_to_const(#obj: #target_type) -> *const #ffi_type {
            //                 #to_presentation
            //             }
            //             unsafe fn destroy(ffi: *mut #ffi_type) {
            //                 #destroy_presentation;
            //             }
            //         }
            //
            //     }
            // }
        }.to_tokens(tokens)
    }
}
