use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;
use syn::{Generics, ReturnType, Type};
use crate::ast::Depunctuated;
use crate::composer::CommaPunctuatedArgs;
use crate::presentation::{DestroyPresentation, DictionaryName, Expansion, FromConversionPresentation, ToConversionPresentation};

#[derive(Clone, Debug)]
pub enum InterfacePresentation {
    Empty,
    Conversion {
        attrs: Depunctuated<Expansion>,
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
        attrs: Depunctuated<Expansion>,
        types: (
            Type, // FFI
            Type // Original
        ),
        decode: TokenStream2,
        encode: TokenStream2,
    },
    Callback {
        attrs: Depunctuated<Expansion>,
        ffi_type: Type,
        inputs: CommaPunctuatedArgs,
        output: ReturnType,
        body: TokenStream2
    }
}

impl ToTokens for InterfacePresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Empty => quote!(),
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

                let package = DictionaryName::Package;
                let interface = DictionaryName::Interface;
                let obj = DictionaryName::Obj;
                quote! {
                    #attrs
                    impl #generic_bounds #package::#interface<#target_type #generic_bounds> for #ffi_type #where_clause {
                        unsafe fn ffi_from_const(ffi: *const #ffi_type) -> #target_type #generic_bounds {
                            #from_presentation
                        }
                        unsafe fn ffi_to_const(#obj: #target_type #generic_bounds) -> *const #ffi_type {
                            #to_presentation
                        }
                        unsafe fn destroy(ffi: *mut #ffi_type) {
                            #destroy_presentation;
                        }
                    }
                }
            },
            Self::VecConversion { attrs, types: (ffi_type, target_type), decode, encode } => quote! {
                #attrs
                impl ferment_interfaces::FFIVecConversion for #ffi_type {
                    type Value = #target_type;
                    unsafe fn decode(&self) -> Self::Value { #decode }
                    unsafe fn encode(obj: Self::Value) -> *mut Self { #encode }
                }
            },
            Self::Callback { attrs, ffi_type, inputs, output, body } => {
                // impl Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String {
                //     pub fn get(&self) -> fn(u32, [u8; 32]) -> Option<String> {
                //         |o_0, o_1| unsafe {
                //             let ffi_result = (self.caller)(o_0, ferment_interfaces::FFIConversion::ffi_to(o_1));
                //             (!ffi_result.is_null()).then(|| {
                //                 let result = <std::os::raw::c_char as ferment_interfaces::FFIConversion<String>>::ffi_from(ffi_result);
                //                 (self.destructor)(ffi_result);
                //                 result
                //             })
                //         }
                //     }
                // }

                quote! {
                    #attrs
                    impl #ffi_type {
                        pub unsafe fn call(&self, #inputs) #output {
                            #body
                        }
                    }
                }
                // match output {
                //     ReturnType::Default => quote! {
                //         #attrs
                //         impl ferment_interfaces::FFICallback<(#inputs), ()> for #ffi_type {
                //             unsafe fn get<T: Fn((#inputs))>(&self) -> T {
                //                 #body
                //             }
                //         }
                //     },
                //     ReturnType::Type(_, output_ty) => quote! {
                //         #attrs
                //         impl ferment_interfaces::FFICallback<(#inputs), #output_ty> for #ffi_type {
                //             unsafe fn get<T: Fn((#inputs)) -> #output_ty>(&self) -> T {
                //                 #body
                //             }
                //         }
                //     }
                // }
            }
        }.to_tokens(tokens)
    }
}
