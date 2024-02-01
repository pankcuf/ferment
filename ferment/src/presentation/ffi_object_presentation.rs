use quote::{quote, ToTokens};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use syn::{Generics, parse_quote};
use std::rc::Rc;
use std::cell::RefCell;
use crate::context::ScopeContext;
use crate::conversion::{FieldTypeConversion, GenericArgPresentation};
use crate::helper::ffi_destructor_name;
use crate::interface::create_struct;
use crate::presentation::{BindingPresentation, DropInterfacePresentation, FromConversionPresentation, ToConversionPresentation};
use crate::presentation::context::OwnedItemPresenterContext;
use crate::presentation::conversion_interface_presentation::ConversionInterfacePresentation;

pub enum FFIObjectPresentation {
    Empty,
    Callback {
        name: TokenStream2,
        arguments: Vec<TokenStream2>,
        output_expression: TokenStream2,
    },
    Function {
        name: TokenStream2,
        arguments: Vec<TokenStream2>,
        input_conversions: TokenStream2,
        output_expression: TokenStream2,
        output_conversions: TokenStream2,
    },
    AsyncFunction {
        name: TokenStream2,
        arguments: Vec<TokenStream2>,
        input_conversions: TokenStream2,
        output_expression: TokenStream2,
        output_conversions: TokenStream2,
    },
    TraitVTable {
        name: TokenStream2,
        fields: Vec<FFIObjectPresentation>
    },
    TraitVTableInnerFn {
        name: TokenStream2,
        name_and_args: TokenStream2,
        output_expression: TokenStream2,
    },
    TraitObject {
        name: TokenStream2,
        vtable_name: TokenStream2,
    },
    Full(TokenStream2),
    Result {
        target_type: TokenStream2,
        ffi_type: Ident,

        ok_presentation: GenericArgPresentation,
        error_presentation: GenericArgPresentation,
        generics: Option<Generics>,
        context: Rc<RefCell<ScopeContext>>
    },
    Map {
        target_type: TokenStream2,
        ffi_type: Ident,

        key_presentation: GenericArgPresentation,
        value_presentation: GenericArgPresentation,
        generics: Option<Generics>,
        context: Rc<RefCell<ScopeContext>>
    },
    Vec {
        target_type: TokenStream2,
        ffi_type: Ident,
        value_presentation: GenericArgPresentation,
        generics: Option<Generics>,
        context: Rc<RefCell<ScopeContext>>
    },
    // Generic {
    //     target_type: TokenStream2,
    //     ffi_type: TokenStream2,
    //     arg_presentations: Vec<TokenStream2>
    // }
}


impl ToTokens for FFIObjectPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Callback { name, arguments, output_expression} =>
                quote! {
                    #[allow(non_camel_case_types)]
                    pub type #name = unsafe extern "C" fn(#(#arguments),*) #output_expression;
                },
            Self::Function { name, arguments, input_conversions, output_expression, output_conversions } => {
                let macros = quote!(#[no_mangle]);
                quote! {
                    #macros
                    pub unsafe extern "C" fn #name (#(#arguments,)*) -> #output_expression {
                        let obj = #input_conversions;
                        #output_conversions
                    }
                }
            },
            Self::AsyncFunction { name, arguments, input_conversions, output_expression, output_conversions } => {
                let macros = quote!(#[no_mangle]);
                quote! {
                    #macros
                    pub unsafe extern "C" fn #name(runtime: *mut std::os::raw::c_void, #(#arguments,)*) -> #output_expression {
                        let rt = unsafe { &*(runtime as *mut tokio::runtime::Runtime) };
                        let obj = rt.block_on(async { #input_conversions .await });
                        #output_conversions
                    }
                }
            },
            FFIObjectPresentation::TraitVTableInnerFn { name, name_and_args, output_expression } => {
                quote!(pub #name: #name_and_args -> #output_expression)
            }
            Self::Full(presentation) => quote!(#presentation),
            Self::TraitVTable { name, fields } => {
                create_struct(quote!(#name), quote!({ #(#fields,)* }))
            },
            Self::TraitObject { name, vtable_name } => {
                create_struct(quote!(#name), quote!({
                    pub object: *const (),
                    pub vtable: *const #vtable_name
                }))
            },
            Self::Result { target_type, ffi_type, ok_presentation, error_presentation, generics, context} => {
                let GenericArgPresentation { ty: ok_type, from_conversion: from_ok_conversion, to_conversion: to_ok_conversion, destructor: ok_destructor } = ok_presentation;
                let GenericArgPresentation { ty: error_type, from_conversion: from_error_conversion, to_conversion: to_error_conversion, destructor: error_destructor } = error_presentation;
                let drop_code = [ok_destructor, error_destructor];
                let object_presentation = create_struct(quote!(#ffi_type), quote!({
                        pub ok: *mut #ok_type,
                        pub error: *mut #error_type,
                    }));
                let conversion_presentation = ConversionInterfacePresentation::Interface {
                    ffi_type: quote!(#ffi_type),
                    target_type: quote!(#target_type),
                    from_presentation: FromConversionPresentation::Result(quote!(#from_ok_conversion), quote!(#from_error_conversion)),
                    to_presentation: ToConversionPresentation::Result(quote!(#to_ok_conversion), quote!(#to_error_conversion)),
                    destroy_presentation: quote!(ferment_interfaces::unbox_any(ffi);),
                    generics: generics.clone()
                };
                let drop_presentation = DropInterfacePresentation::Full(quote!(#ffi_type), quote!(#(#drop_code)*));
                let ok_conversion = FieldTypeConversion::Named(quote!(ok), parse_quote!(*mut #ok_type));
                let error_conversion = FieldTypeConversion::Named(quote!(error), parse_quote!(*mut #error_type));
                let bindings = vec![
                    BindingPresentation::Constructor {
                        ffi_ident: ffi_type.clone(),
                        ctor_arguments: vec![
                            OwnedItemPresenterContext::Named(ok_conversion, false),
                            OwnedItemPresenterContext::Named(error_conversion, false),
                        ],
                        body_presentation: quote!({ ok, error }),
                        context: Rc::clone(context)
                    },
                    BindingPresentation::Destructor {
                        ffi_name: quote!(#ffi_type),
                        destructor_ident: ffi_destructor_name(ffi_type).to_token_stream()
                    }
                ];
                quote! {
                    #object_presentation
                    #conversion_presentation
                    #drop_presentation
                    #(#bindings)*
                }
            },
            Self::Map { target_type, ffi_type, key_presentation, value_presentation, generics, context} => {
                let GenericArgPresentation { ty: key, from_conversion: from_key_conversion, to_conversion: to_key_conversion, destructor: key_destructor } = key_presentation;
                let GenericArgPresentation { ty: value, from_conversion: from_value_conversion, to_conversion: to_value_conversion, destructor: value_destructor } = value_presentation;
                let drop_code = [key_destructor, value_destructor];

                let object_presentation = create_struct(quote!(#ffi_type), quote!({
                        pub count: usize,
                        pub keys: *mut #key,
                        pub values: *mut #value,
                    }));
                let conversion_presentation = ConversionInterfacePresentation::Interface {
                    ffi_type: quote!(#ffi_type),
                    target_type: quote!(#target_type),
                    from_presentation: FromConversionPresentation::Map(quote!(#from_key_conversion), quote!(#from_value_conversion)),
                    to_presentation: ToConversionPresentation::Map(quote!(#to_key_conversion), quote!(#to_value_conversion)),
                    destroy_presentation: quote!(ferment_interfaces::unbox_any(ffi);),
                    generics: generics.clone()
                };
                let drop_presentation = DropInterfacePresentation::Full(quote!(#ffi_type), quote!(#(#drop_code)*));
                let key_conversion = FieldTypeConversion::Named(quote!(keys), parse_quote!(*mut #key));
                let value_conversion = FieldTypeConversion::Named(quote!(values), parse_quote!(*mut #value));
                let count_conversion = FieldTypeConversion::Named(quote!(count), parse_quote!(usize));
                let bindings = vec![
                    BindingPresentation::Constructor {
                        ffi_ident: ffi_type.clone(),
                        ctor_arguments: vec![
                            OwnedItemPresenterContext::Named(key_conversion, false),
                            OwnedItemPresenterContext::Named(value_conversion, false),
                            OwnedItemPresenterContext::Named(count_conversion, false),
                        ],
                        body_presentation: quote!({ count, keys, values }),
                        context: Rc::clone(context)
                    },
                    BindingPresentation::Destructor {
                        ffi_name: quote!(#ffi_type),
                        destructor_ident: ffi_destructor_name(ffi_type).to_token_stream()
                    }
                ];
                quote! {
                    #object_presentation
                    #conversion_presentation
                    #drop_presentation
                    #(#bindings)*
                }
            },
            Self::Vec { target_type, ffi_type, value_presentation, generics, context } => {
                let GenericArgPresentation { ty: value, from_conversion: from_value_conversion, to_conversion: to_value_conversion, destructor: value_destructor } = value_presentation;
                let drop_code = [value_destructor];

                let conversion_presentation = ConversionInterfacePresentation::Interface {
                    ffi_type: quote!(#ffi_type),
                    target_type: quote!(#target_type),
                    from_presentation: FromConversionPresentation::Vec,
                    to_presentation: ToConversionPresentation::Vec,
                    destroy_presentation: quote!(ferment_interfaces::unbox_any(ffi);),
                    generics: generics.clone()
                };
                let object_presentation = create_struct(quote!(#ffi_type), quote!({
                        pub count: usize,
                        pub values: *mut #value,
                    }));
                let drop_presentation = DropInterfacePresentation::Full(ffi_type.to_token_stream(), quote!(#(#drop_code)*));
                let value_conversion = FieldTypeConversion::Named(quote!(values), parse_quote!(*mut #value));
                let count_conversion = FieldTypeConversion::Named(quote!(count), parse_quote!(usize));
                let bindings = vec![
                    BindingPresentation::Constructor {
                        ffi_ident: ffi_type.clone(),
                        ctor_arguments: vec![
                            OwnedItemPresenterContext::Named(value_conversion, false),
                            OwnedItemPresenterContext::Named(count_conversion, false),
                        ],
                        body_presentation: quote!({ count, values }),
                        context: Rc::clone(context),
                    },
                    BindingPresentation::Destructor {
                        ffi_name: quote!(#ffi_type),
                        destructor_ident: ffi_destructor_name(ffi_type).to_token_stream()
                    }
                ];
                quote! {
                    #object_presentation
                    #conversion_presentation
                    impl ferment_interfaces::FFIVecConversion for #ffi_type {
                        type Value = #target_type;
                        unsafe fn decode(&self) -> Self::Value { #from_value_conversion }
                        unsafe fn encode(obj: Self::Value) -> *mut Self { #to_value_conversion }
                    }
                    #drop_presentation
                    #(#bindings)*
                }
            },
            // FFIObjectPresentation::Generic { .. } => {}
            FFIObjectPresentation::Empty => { /* Box<T> */
                quote!()
            }
        }.to_tokens(tokens)
    }
}
