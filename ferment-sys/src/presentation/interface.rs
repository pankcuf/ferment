use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;
use syn::{Attribute, GenericParam, Generics, Lifetime, LifetimeParam, ReturnType, Type};
use crate::ast::{CommaPunctuated, CommaPunctuatedTokens};
use crate::composer::{CommaPunctuatedArgs, TypePair};
use crate::presentation::{DictionaryExpr, DictionaryName, InterfacesMethodExpr};

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum InterfacePresentation {
    Empty,
    Ctor {
        attrs: Vec<Attribute>,
        generics: Option<Generics>,
        lifetimes: Vec<Lifetime>,
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
            Option<Generics>,
            Vec<Lifetime>
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
            Option<Generics>,
            Vec<Lifetime>
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
        lifetimes: Vec<Lifetime>,
        body: TokenStream2,
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
    pub fn conversion_from_root<T: ToTokens>(attrs: &Vec<Attribute>, types: &TypePair, body: T, generics: &Option<Generics>, lifetimes: &Vec<Lifetime>) -> Self {
        Self::conversion_from(attrs, types, DictionaryExpr::FromRoot(body.to_token_stream()), generics, lifetimes)
    }
    pub fn conversion_to_boxed<T: ToTokens>(attrs: &Vec<Attribute>, types: &TypePair, body: T, generics: &Option<Generics>, lifetimes: &Vec<Lifetime>) -> Self {
        Self::conversion_to(attrs, types, InterfacesMethodExpr::Boxed(body.to_token_stream()), generics, lifetimes)
    }
    pub fn conversion_to_boxed_self_destructured<T: ToTokens>(attrs: &Vec<Attribute>, types: &TypePair, body: T, generics: &Option<Generics>, lifetimes: &Vec<Lifetime>) -> Self {
        Self::conversion_to_boxed(attrs, types, DictionaryExpr::self_destruct(body), generics, lifetimes)
    }
    pub fn conversion_from<T: ToTokens>(attrs: &Vec<Attribute>, types: &TypePair, method_body: T, generics: &Option<Generics>, lifetimes: &Vec<Lifetime>) -> Self {
        InterfacePresentation::ConversionFrom {
            attrs: attrs.clone(),
            types: types.clone(),
            conversions: (method_body.to_token_stream(), generics.clone(), lifetimes.clone())
        }
    }
    pub fn conversion_to<T: ToTokens>(attrs: &Vec<Attribute>, types: &TypePair, method_body: T, generics: &Option<Generics>, lifetimes: &Vec<Lifetime>) -> Self {
        InterfacePresentation::ConversionTo {
            attrs: attrs.clone(),
            types: types.clone(),
            conversions: (method_body.to_token_stream(), generics.clone(), lifetimes.clone())
        }
    }
    pub fn drop<T: ToTokens>(attrs: &Vec<Attribute>, ty: Type, body: T) -> Self {
        InterfacePresentation::Drop { attrs: attrs.clone(), ty, body: body.to_token_stream() }
    }

    pub fn callback<T: ToTokens, U: ToTokens>(attrs: &Vec<Attribute>, ffi_type: &Type, inputs: CommaPunctuatedArgs, output: ReturnType, lifetimes: &Vec<Lifetime>, args_conversions: T, result_conversion: U) -> Self {
        InterfacePresentation::Callback {
            attrs: attrs.clone(),
            ffi_type: ffi_type.clone(),
            inputs,
            output,
            lifetimes: lifetimes.clone(),
            body: DictionaryExpr::CallbackCaller(args_conversions.to_token_stream(), result_conversion.to_token_stream()).to_token_stream(),
        }
    }
    pub fn send_sync(attrs: &Vec<Attribute>, ffi_type: &Type) -> Self {
        InterfacePresentation::SendAndSync { attrs: attrs.clone(), ffi_type: ffi_type.clone() }
    }
    pub fn vec(attrs: &Vec<Attribute>, types: &TypePair, decode: TokenStream2, encode: TokenStream2) -> Self {
        InterfacePresentation::VecConversion { attrs: attrs.clone(), types: types.clone(), decode, encode }
    }
}

fn generics_presentation(generics: &Option<Generics>, lifetimes: &Vec<Lifetime>) -> (TokenStream2, TokenStream2) {
    let result = match generics {
        Some(generics) => {
            let mut params = CommaPunctuated::from_iter(lifetimes.iter().map(|lt| GenericParam::Lifetime(LifetimeParam {
                attrs: vec![],
                lifetime: lt.clone(),
                colon_token: None,
                bounds: Default::default(),
            })));
            params.extend(generics.params.clone());

            let gens = params.iter().map(ToTokens::to_token_stream);
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
        None => {
            let lifetimes = CommaPunctuated::from_iter(lifetimes.iter().map(|lt| GenericParam::Lifetime(LifetimeParam {
                attrs: vec![],
                lifetime: lt.clone(),
                colon_token: None,
                bounds: Default::default(),
            })));
            let bounds = if lifetimes.is_empty() { quote!() } else { quote!(<#lifetimes>) };
            (bounds, quote!())
        }
    };
    result
}

impl ToTokens for InterfacePresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Empty =>
                quote!(),
            Self::Ctor { attrs, ffi_type, generics, lifetimes, args, presentation } => {
                let (generic_bounds, where_clause) = generics_presentation(generics, lifetimes);
                quote! {
                    #(#attrs)*
                    impl #generic_bounds #ffi_type #where_clause {
                        pub fn new(#args) -> Self {
                            #presentation
                        }
                    }
                }
            },
            Self::ConversionFrom {
                attrs,
                types: (ffi_type, target_type),
                conversions: (presentation, generics, lifetimes),
            } => {
                let (generic_bounds, where_clause) = generics_presentation(generics, lifetimes);
                let package = DictionaryName::Package;
                let interface_from = DictionaryName::InterfaceFrom;
                quote! {
                    #(#attrs)*
                    impl #generic_bounds #package::#interface_from<#target_type> for #ffi_type #where_clause {
                        unsafe fn ffi_from_const(ffi: *const #ffi_type) -> #target_type {
                            #presentation
                        }
                    }
                }
            },
            Self::ConversionTo {
                attrs,
                types: (ffi_type, target_type),
                conversions: (presentation, generics, lifetimes),
            } => {
                let (generic_bounds, where_clause) = generics_presentation(generics, lifetimes);
                let package = DictionaryName::Package;
                let interface_to = DictionaryName::InterfaceTo;
                let obj = DictionaryName::Obj;
                quote! {
                    #(#attrs)*
                    impl #generic_bounds #package::#interface_to<#target_type> for #ffi_type #where_clause {
                        unsafe fn ffi_to_const(#obj: #target_type) -> *const #ffi_type {
                            #presentation
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
            Self::Callback { attrs, ffi_type, inputs, output, lifetimes, body } => {
                let lifetimes = CommaPunctuated::from_iter(lifetimes.iter().map(|lt| GenericParam::Lifetime(LifetimeParam {
                    attrs: vec![],
                    lifetime: lt.clone(),
                    colon_token: None,
                    bounds: Default::default(),
                })));
                let bounds = if lifetimes.is_empty() { quote!() } else { quote!(<#lifetimes>) };
                quote! {
                    #(#attrs)*
                    impl #ffi_type {
                        pub unsafe fn call #bounds(&self, #inputs) #output {
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
