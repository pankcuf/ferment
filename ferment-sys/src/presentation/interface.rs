use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;
use syn::{Attribute, Generics, ReturnType, Type};
use crate::ast::CommaPunctuatedTokens;
use crate::composer::CommaPunctuatedArgs;
use crate::ext::Terminated;
use crate::presentation::{DictionaryExpr, DictionaryName, InterfacesMethodExpr};

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum InterfacePresentation {
    Empty,
    Ctor {
        attrs: Vec<Attribute>,
        generics: Option<Generics>,
        ffi_type: Type,
        args: CommaPunctuatedTokens,
        presentation: TokenStream2,
    },
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
        inputs: CommaPunctuatedArgs,
        output: ReturnType,
        body: TokenStream2
    },
    SendAndSync {
        attrs: Vec<Attribute>,
        ffi_type: Type,
    },
    Drop {
        attrs: Vec<Attribute>,
        ty: Type,
        body: TokenStream2
    }
}

impl InterfacePresentation {
    pub fn conversion_from_root<T: ToTokens>(attrs: &Vec<Attribute>, types: &(Type, Type), body: T, generics: &Option<Generics>) -> Self {
        Self::conversion_from(attrs, types, DictionaryExpr::FromRoot(body.to_token_stream()), generics)
    }
    pub fn conversion_to_boxed<T: ToTokens>(attrs: &Vec<Attribute>, types: &(Type, Type), body: T, generics: &Option<Generics>) -> Self {
        Self::conversion_to(attrs, types, InterfacesMethodExpr::Boxed(body.to_token_stream()), generics)
    }
    pub fn conversion_to_boxed_self_destructured<T: ToTokens>(attrs: &Vec<Attribute>, types: &(Type, Type), body: T, generics: &Option<Generics>) -> Self {
        Self::conversion_to_boxed(attrs, types, DictionaryExpr::SelfDestructuring(body.to_token_stream()), generics)
    }
    pub fn conversion_unbox_any_terminated<T: ToTokens>(attrs: &Vec<Attribute>, types: &(Type, Type), body: T, generics: &Option<Generics>) -> Self {
        Self::conversion_destroy(attrs, types, InterfacesMethodExpr::UnboxAny(body.to_token_stream()).to_token_stream().terminated(), generics)
    }

    pub fn conversion_from<T: ToTokens>(attrs: &Vec<Attribute>, types: &(Type, Type), conversions: T, generics: &Option<Generics>) -> Self {
        InterfacePresentation::ConversionFrom {
            attrs: attrs.clone(),
            types: types.clone(),
            conversions: (conversions.to_token_stream(), generics.clone())
        }
    }
    pub fn conversion_to<T: ToTokens>(attrs: &Vec<Attribute>, types: &(Type, Type), conversions: T, generics: &Option<Generics>) -> Self {
        InterfacePresentation::ConversionTo {
            attrs: attrs.clone(),
            types: types.clone(),
            conversions: (conversions.to_token_stream(), generics.clone())
        }
    }
    pub fn conversion_destroy<T: ToTokens>(attrs: &Vec<Attribute>, types: &(Type, Type), conversions: T, generics: &Option<Generics>) -> Self {
        InterfacePresentation::ConversionDestroy {
            attrs: attrs.clone(),
            types: types.clone(),
            conversions: (conversions.to_token_stream(), generics.clone())
        }
    }
    pub fn drop<T: ToTokens>(attrs: &Vec<Attribute>, ty: Type, body: T) -> Self {
        InterfacePresentation::Drop { attrs: attrs.clone(), ty, body: body.to_token_stream() }
    }

    pub fn callback<T: ToTokens, U: ToTokens>(attrs: &Vec<Attribute>, ffi_type: &Type, inputs: CommaPunctuatedArgs, output: ReturnType, args_conversions: T, result_conversion: U) -> Self {
        InterfacePresentation::Callback {
            attrs: attrs.clone(),
            ffi_type: ffi_type.clone(),
            inputs,
            output,
            body: DictionaryExpr::CallbackCaller(args_conversions.to_token_stream(), result_conversion.to_token_stream()).to_token_stream(),
        }
    }
    pub fn send_sync(attrs: &Vec<Attribute>, ffi_type: &Type) -> Self {
        InterfacePresentation::SendAndSync { attrs: attrs.clone(), ffi_type: ffi_type.clone() }
    }
    pub fn vec(attrs: &Vec<Attribute>, types: &(Type, Type), decode: TokenStream2, encode: TokenStream2) -> Self {
        InterfacePresentation::VecConversion { attrs: attrs.clone(), types: types.clone(), decode, encode }
    }
}

fn generics_presentation(generics: &Option<Generics>) -> (TokenStream2, TokenStream2) {
    match generics {
        Some(generics) => {
            let gens = generics.params.iter().map(ToTokens::to_token_stream);
            let where_clause = match &generics.where_clause {
                Some(where_clause) => {
                    let where_predicates = where_clause.predicates.iter().map(ToTokens::to_token_stream);
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
            Self::Empty =>
                quote!(),
            Self::Ctor { attrs, ffi_type, generics, args, presentation } => {
                let (generic_bounds, where_clause) = generics_presentation(generics);
                quote! {
                    #(#attrs)*
                    impl #generic_bounds #ffi_type #where_clause {
                        pub fn new(#args) -> Self {
                            // Self ( #arg_names )
                            // Self { #arg_names }
                            #presentation
                        }
                    }
                }
            },
            Self::ConversionFrom {
                attrs,
                types: (ffi_type, target_type),
                conversions: (presentation, generics),
            } => {
                let (generic_bounds, where_clause) = generics_presentation(generics);
                let package = DictionaryName::Package;
                let interface_from = DictionaryName::InterfaceFrom;
                quote! {
                    #(#attrs)*
                    impl #generic_bounds #package::#interface_from<#target_type #generic_bounds> for #ffi_type #where_clause {
                        unsafe fn ffi_from_const(ffi: *const #ffi_type) -> #target_type #generic_bounds {
                            #presentation
                        }
                    }
                }
            },
            Self::ConversionTo {
                attrs,
                types: (ffi_type, target_type),
                conversions: (presentation, generics),
            } => {
                let (generic_bounds, where_clause) = generics_presentation(generics);
                let package = DictionaryName::Package;
                let interface_to = DictionaryName::InterfaceTo;
                let obj = DictionaryName::Obj;
                quote! {
                    #(#attrs)*
                    impl #generic_bounds #package::#interface_to<#target_type #generic_bounds> for #ffi_type #where_clause {
                        unsafe fn ffi_to_const(#obj: #target_type #generic_bounds) -> *const #ffi_type {
                            #presentation
                        }
                    }
                }
            },
            Self::ConversionDestroy {
                attrs,
                types: (ffi_type, target_type),
                conversions: (presentation, generics),
            } => {
                let (generic_bounds, where_clause) = generics_presentation(generics);
                let package = DictionaryName::Package;
                let interface_destroy = DictionaryName::InterfaceDestroy;
                quote! {
                    #(#attrs)*
                    impl #generic_bounds #package::#interface_destroy<#target_type #generic_bounds> for #ffi_type #where_clause {
                        unsafe fn destroy(ffi: *mut #ffi_type) {
                            #presentation;
                        }
                    }
                }
            },
            Self::VecConversion {
                attrs,
                types: (ffi_type, target_type),
                decode,
                encode } => quote! {
                #(#attrs)*
                impl ferment::FFIVecConversion for #ffi_type {
                    type Value = #target_type;
                    unsafe fn decode(&self) -> Self::Value { #decode }
                    unsafe fn encode(obj: Self::Value) -> *mut Self { #encode }
                }
            },
            Self::Callback { attrs, ffi_type, inputs, output, body } => {
                quote! {
                    #(#attrs)*
                    impl #ffi_type {
                        pub unsafe fn call(&self, #inputs) #output {
                            #body
                        }
                    }

                }
            }
            Self::SendAndSync { attrs, ffi_type } => {
                quote! {
                    #(#attrs)*
                    unsafe impl Send for #ffi_type {}
                    #(#attrs)*
                    unsafe impl Sync for #ffi_type {}
                }
            },
            Self::Drop { attrs, ty, body } => quote! {
                #(#attrs)*
                impl Drop for #ty { fn drop(&mut self) { unsafe { #body; } } }
            }
        }.to_tokens(tokens)
    }
}
