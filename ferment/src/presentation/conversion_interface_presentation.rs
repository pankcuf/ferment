use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;
use syn::{Attribute, Generics, ReturnType, Type};
use crate::composer::CommaPunctuatedArgs;
use crate::presentation::DictionaryName;

// pub struct Presentable<T: ?Sized + Clone + ToTokens>(pub T);
#[derive(Clone, Debug)]
pub enum InterfacePresentation {
    // Empty,
    ConversionFrom {
        attrs: Vec<Attribute>,
        types: (
            Type, // FFI
            Type // Original
        ),
        conversions: (
            TokenStream2,
            Option<Generics>
        ),
    },
    ConversionTo {
        attrs: Vec<Attribute>,
        types: (
            Type, // FFI
            Type // Original
        ),
        conversions: (
            TokenStream2,
            Option<Generics>
        ),
    },
    ConversionDestroy {
        attrs: Vec<Attribute>,
        types: (
            Type, // FFI
            Type // Original
        ),
        conversions: (
            TokenStream2,
            Option<Generics>
        ),
    },
    VecConversion {
        attrs: Vec<Attribute>,
        types: (
            Type, // FFI
            Type // Original
        ),
        decode: TokenStream2,
        encode: TokenStream2,
    },
    Callback {
        attrs: Vec<Attribute>,
        ffi_type: Type,
        // ffi_output: ReturnType,
        // ffi_args: CommaPunctuated<TokenStream2>,
        inputs: CommaPunctuatedArgs,
        output: ReturnType,
        body: TokenStream2
    },
    CallbackNew {
        attrs: Vec<Attribute>,
        ffi_type: Type,
        inputs: CommaPunctuatedArgs,
        output: ReturnType,
        body: TokenStream2
    },
    SendAndSync {
        attrs: Vec<Attribute>,
        ffi_type: Type,
    }
}

fn generics_presentation(generics: &Option<Generics>) -> (TokenStream2, TokenStream2) {
    match generics {
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
    }
}

impl ToTokens for InterfacePresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            // Self::Empty => quote!(),
            Self::ConversionFrom {
                attrs,
                types: (
                    ffi_type,
                    target_type),
                conversions: (
                    from_presentation,
                    generics),
            } => {
                let (generic_bounds, where_clause) = generics_presentation(generics);
                let package = DictionaryName::Package;
                let interface_from = DictionaryName::InterfaceFrom;
                quote! {
                    #(#attrs)*
                    impl #generic_bounds #package::#interface_from<#target_type #generic_bounds> for #ffi_type #where_clause {
                        unsafe fn ffi_from_const(ffi: *const #ffi_type) -> #target_type #generic_bounds {
                            #from_presentation
                        }
                    }
                }
            },
            Self::ConversionTo {
                attrs,
                types: (
                    ffi_type,
                    target_type),
                conversions: (
                    to_presentation,
                    generics),
            } => {
                let (generic_bounds, where_clause) = generics_presentation(generics);
                let package = DictionaryName::Package;
                let interface_to = DictionaryName::InterfaceTo;
                let obj = DictionaryName::Obj;
                quote! {
                    #(#attrs)*
                    impl #generic_bounds #package::#interface_to<#target_type #generic_bounds> for #ffi_type #where_clause {
                        unsafe fn ffi_to_const(#obj: #target_type #generic_bounds) -> *const #ffi_type {
                            #to_presentation
                        }
                    }
                }
            },
            Self::ConversionDestroy {
                attrs,
                types: (
                    ffi_type,
                    target_type),
                conversions: (
                    destroy_presentation,
                    generics),
            } => {
                let (generic_bounds, where_clause) = generics_presentation(generics);
                let package = DictionaryName::Package;
                let interface_destroy = DictionaryName::InterfaceDestroy;
                quote! {
                    #(#attrs)*
                    impl #generic_bounds #package::#interface_destroy<#target_type #generic_bounds> for #ffi_type #where_clause {
                        unsafe fn destroy(ffi: *mut #ffi_type) {
                            #destroy_presentation;
                        }
                    }
                }
            },
            Self::VecConversion { attrs, types: (ffi_type, target_type), decode, encode } => quote! {
                #(#attrs)*
                impl ferment_interfaces::FFIVecConversion for #ffi_type {
                    type Value = #target_type;
                    unsafe fn decode(&self) -> Self::Value { #decode }
                    unsafe fn encode(obj: Self::Value) -> *mut Self { #encode }
                }
            },
            Self::CallbackNew { attrs, ffi_type, inputs, output, body } => {
                quote! {
                    #(#attrs)*
                    impl #ffi_type {
                        pub unsafe fn call(&self, #inputs) #output {
                            #body
                        }
                    }
                }
            },
            Self::Callback { attrs, ffi_type, inputs, output, body } => {
                // impl Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String {
                //     pub fn get(&self) -> fn(u32, [u8; 32]) -> Option<String> {
                //         |o_0, o_1| unsafe {
                //             let ffi_result = (self.caller)(o_0, ferment_interfaces::FFIConversionTo::ffi_to(o_1));
                //             (!ffi_result.is_null()).then(|| {
                //                 let result = <std::os::raw::c_char as ferment_interfaces::FFIConversionFrom<String>>::ffi_from(ffi_result);
                //                 (self.destructor)(ffi_result);
                //                 result
                //             })
                //         }
                //     }
                // }
                // unsafe extern "C" fn caller(context: *const FFIContext, quorum_type: u32, quorum_hash: *mut c_char, core_chain_locked_height: u32) -> *mut c_char {
                //     FFIConversionTo::ffi_to(((&*context).caller)(context, quorum_type, FFIConversionFrom::ffi_from(quorum_hash), core_chain_locked_height))
                // }
                // unsafe extern "C" fn destructor(result: *mut c_char) {
                //     unbox_any(result);
                // }

                // let destructor = match ffi_output {
                //     ReturnType::Default => quote! {},
                //     ReturnType::Type(_, ty) => quote! {
                //         pub unsafe extern "C" fn destructor(context: *const ferment_example_thread_safe::entry::FFIContext, result: #ty) {
                //             ((&*context).destructor)(result);
                //         }
                //     }
                // };

                quote! {
                    #(#attrs)*
                    impl #ffi_type {
                        pub unsafe fn call(&self, #inputs) #output {
                            #body
                        }
                        // pub unsafe extern "C" fn caller(#ffi_args) #ffi_output {
                        //     ferment_interfaces::FFIConversionTo::ffi_to(((&*o_0).caller)(o_0, o_1, ferment_interfaces::FFIConversionFrom::ffi_from (o_2)))
                        // }
                        // #destructor
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
            InterfacePresentation::SendAndSync { attrs, ffi_type } => {
                quote! {
                    #(#attrs)*
                    unsafe impl Send for #ffi_type {}
                    #(#attrs)*
                    unsafe impl Sync for #ffi_type {}
                }
            }
        }.to_tokens(tokens)
    }
}
