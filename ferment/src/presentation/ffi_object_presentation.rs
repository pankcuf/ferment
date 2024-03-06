use quote::{quote, ToTokens};
use proc_macro2::{TokenStream as TokenStream2};
use syn::{Generics, parse_quote, Path, Type};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma};
use crate::composer::{ConstructorPresentableContext, Depunctuated, ParentComposer};
use crate::composition::TraitVTableMethodComposition;
use crate::context::ScopeContext;
use crate::conversion::{FieldTypeConversion, GenericArgPresentation};
use crate::ext::{Accessory, Mangle};
use crate::wrapped::Wrapped;
use crate::interface::create_struct;
use crate::naming::{DictionaryFieldName, Name};
use crate::presentation::{BindingPresentation, DropInterfacePresentation, FromConversionPresentation, ScopeContextPresentable, ToConversionPresentation};
use crate::presentation::context::{IteratorPresentationContext, OwnedItemPresentableContext};
use crate::presentation::context::binding::BindingPresentableContext;
use crate::presentation::conversion_interface_presentation::ConversionInterfacePresentation;

pub enum FFIObjectPresentation {
    Empty,
    StaticVTable {
        name: Name,
        methods_compositions: Vec<TraitVTableMethodComposition>,
        // methods_names: Vec<Ident>,
        // methods_signatures: Vec<TokenStream2>,
        fq_trait_vtable: TokenStream2,
        // methods_implementations: Vec<TraitVTablePresentation>,
        // methods_declarations: Vec<TraitVTablePresentation>,
    },
    TraitVTable {
        name: Name,
        fields: Punctuated<BindingPresentation, Comma>
    },
    // TraitVTableInnerFn {
    //     name: Name,
    //     name_and_args: TokenStream2,
    //     output_expression: ReturnType,
    // },
    TraitObject {
        name: Name,
        vtable_name: Name,
    },
    Full(TokenStream2),
    // Generic {
    //     target_type: Name,
    //     ffi_type: Name,
    //     presentations: Depunctuated<GenericArgPresentation>,
    //     generics: Option<Generics>,
    //     context: ParentComposer<ScopeContext>
    // },

    Result {
        target_type: Type,
        ffi_type: Type,

        ok_presentation: GenericArgPresentation,
        error_presentation: GenericArgPresentation,

        generics: Option<Generics>,
        context: ParentComposer<ScopeContext>
    },
    Map {
        target_type: Type,
        ffi_type: Type,

        key_presentation: GenericArgPresentation,
        value_presentation: GenericArgPresentation,

        generics: Option<Generics>,
        context: ParentComposer<ScopeContext>
    },
    Vec {
        target_type: Type,
        ffi_type: Type,

        value_presentation: GenericArgPresentation,

        generics: Option<Generics>,
        context: ParentComposer<ScopeContext>
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
            Self::Full(presentation) => quote!(#presentation),
            Self::TraitVTable { name, fields } =>
                create_struct(&parse_quote!(#name), quote!({ #fields })),
            Self::TraitObject { name, vtable_name } =>
                create_struct(&parse_quote!(#name), quote!({ pub object: *const (), pub vtable: *const #vtable_name })),
            // Self::Generic { target_type, ffi_type, presentations, generics, context } => {
            //     quote!()
            // },
            Self::Result { target_type, ffi_type, ok_presentation, error_presentation, generics, context} => {
                let GenericArgPresentation { ty: ok_type, from_conversion: from_ok_conversion, to_conversion: to_ok_conversion, destructor: ok_destructor } = ok_presentation;
                let GenericArgPresentation { ty: error_type, from_conversion: from_error_conversion, to_conversion: to_error_conversion, destructor: error_destructor } = error_presentation;
                let ffi_name = ffi_type.to_mangled_ident_default();
                let ffi_as_path: Path = parse_quote!(#ffi_name);
                let ffi_as_type: Type = parse_quote!(#ffi_name);
                let drop_code = Depunctuated::from_iter([ok_destructor, error_destructor]);
                let source = context.borrow();
                let items = Punctuated::<_, Comma>::from_iter([
                    quote!(pub ok: *mut #ok_type),
                    quote!(pub error: *mut #error_type),
                ]);
                let object_presentation = create_struct(&ffi_as_path, Wrapped::<_, Brace>::new(items.present(&source)).to_token_stream());
                let conversion_presentation = ConversionInterfacePresentation::Interface {
                    types: (ffi_as_type.clone(), target_type.clone()),
                    conversions: (
                        FromConversionPresentation::Result(quote!(#from_ok_conversion), quote!(#from_error_conversion)),
                        ToConversionPresentation::Result(quote!(#to_ok_conversion), quote!(#to_error_conversion)),
                        quote!(ferment_interfaces::unbox_any(ffi);),
                        generics.clone()
                    )
                };
                let drop_presentation = DropInterfacePresentation::Full {
                    ty: ffi_as_type.clone(),
                    body: drop_code.to_token_stream()
                };
                let ok_conversion = FieldTypeConversion::Named(Name::Dictionary(DictionaryFieldName::Ok), ok_type.joined_mut());
                let error_conversion = FieldTypeConversion::Named(Name::Dictionary(DictionaryFieldName::Error), error_type.joined_mut());
                let bindings = Depunctuated::from_iter([
                    BindingPresentableContext::Constructor(
                        ConstructorPresentableContext::Default(Name::Constructor(ffi_as_type.clone()), ffi_as_type.clone()),
                        Punctuated::from_iter([
                            OwnedItemPresentableContext::Named(ok_conversion.clone(), false),
                            OwnedItemPresentableContext::Named(error_conversion.clone(), false)
                        ]),
                        IteratorPresentationContext::Curly(Punctuated::from_iter([
                            OwnedItemPresentableContext::DefaultField(ok_conversion),
                            OwnedItemPresentableContext::DefaultField(error_conversion),
                        ]))
                    ),
                    BindingPresentableContext::Destructor(ffi_as_type.clone())
                ]).present(&source);
                quote! {
                    #object_presentation
                    #conversion_presentation
                    #drop_presentation
                    #bindings
                }
            },
            Self::Map { target_type, ffi_type, key_presentation, value_presentation, generics, context} => {
                println!("Self::Map {} -> {} [{}, {}]", target_type.to_token_stream(), ffi_type.to_token_stream(), key_presentation.ty.to_token_stream(), value_presentation.ty.to_token_stream());
                let ffi_name = ffi_type.to_mangled_ident_default();
                let ffi_as_path: Path = parse_quote!(#ffi_name);
                let ffi_as_type: Type = parse_quote!(#ffi_name);
                let GenericArgPresentation { ty: key, from_conversion: from_key_conversion, to_conversion: to_key_conversion, destructor: key_destructor } = key_presentation;
                let GenericArgPresentation { ty: value, from_conversion: from_value_conversion, to_conversion: to_value_conversion, destructor: value_destructor } = value_presentation;
                let drop_code = Depunctuated::from_iter([key_destructor, value_destructor]);
                let source = context.borrow();
                let items = Punctuated::<_, Comma>::from_iter([
                    quote!(pub count: usize),
                    quote!(pub keys: *mut #key),
                    quote!(pub values: *mut #value),
                ]);

                let object_presentation = create_struct(&ffi_as_path, Wrapped::<_, Brace>::new(items.present(&source)).to_token_stream());
                let conversion_presentation = ConversionInterfacePresentation::Interface {
                    types: (ffi_as_type.clone(), target_type.clone()),
                    conversions: (
                        FromConversionPresentation::Map(quote!(#from_key_conversion), quote!(#from_value_conversion)),
                        ToConversionPresentation::Map(quote!(#to_key_conversion), quote!(#to_value_conversion)),
                        quote!(ferment_interfaces::unbox_any(ffi);),
                        generics.clone()
                    )
                };
                let drop_presentation = DropInterfacePresentation::Full { ty: ffi_as_type.clone(), body: drop_code.to_token_stream() };
                let count_conversion = FieldTypeConversion::Named(Name::Dictionary(DictionaryFieldName::Count), parse_quote!(usize));
                let key_conversion = FieldTypeConversion::Named(Name::Dictionary(DictionaryFieldName::Keys), parse_quote!(*mut #key));
                let value_conversion = FieldTypeConversion::Named(Name::Dictionary(DictionaryFieldName::Values), parse_quote!(*mut #value));
                let bindings = Depunctuated::from_iter([
                    BindingPresentableContext::Constructor(
                        ConstructorPresentableContext::Default(Name::Constructor(ffi_type.clone()), ffi_as_type.clone()),
                        Punctuated::from_iter([
                            OwnedItemPresentableContext::Named(key_conversion.clone(), false),
                            OwnedItemPresentableContext::Named(value_conversion.clone(), false),
                            OwnedItemPresentableContext::Named(count_conversion.clone(), false)
                        ]),
                        IteratorPresentationContext::Curly(Punctuated::from_iter([
                            OwnedItemPresentableContext::DefaultField(count_conversion),
                            OwnedItemPresentableContext::DefaultField(key_conversion),
                            OwnedItemPresentableContext::DefaultField(value_conversion),
                        ]))
                    ),
                    BindingPresentableContext::Destructor(ffi_as_type.clone())
                ]).present(&source);
                quote! {
                    #object_presentation
                    #conversion_presentation
                    #drop_presentation
                    #bindings
                }
            },
            Self::Vec { target_type, ffi_type, value_presentation, generics, context } => {
                let GenericArgPresentation { ty: value, from_conversion: from_value_conversion, to_conversion: to_value_conversion, destructor: value_destructor } = value_presentation;
                let source = context.borrow();
                let drop_code = Depunctuated::from_iter([value_destructor]);
                let ffi_name = ffi_type.to_mangled_ident_default();
                let ffi_as_path: Path = parse_quote!(#ffi_name);
                let ffi_as_type: Type = parse_quote!(#ffi_name);

                let conversion_presentation = ConversionInterfacePresentation::Interface {
                    types: (ffi_as_type.clone(), target_type.clone()),
                    conversions: (
                        FromConversionPresentation::Vec,
                        ToConversionPresentation::Vec,
                        quote!(ferment_interfaces::unbox_any(ffi);),
                        generics.clone())
                };
                let items = Punctuated::<_, Comma>::from_iter([
                    quote!(pub count: usize),
                    quote!(pub values: *mut #value),
                ]);
                let object_presentation = create_struct(&ffi_as_path, Wrapped::<_, Brace>::new(items.present(&source)).to_token_stream());
                let drop_presentation = DropInterfacePresentation::Full { ty: ffi_as_type.clone(), body: drop_code.to_token_stream() };
                let count_conversion = FieldTypeConversion::Named(Name::Dictionary(DictionaryFieldName::Count), parse_quote!(usize));
                let value_conversion = FieldTypeConversion::Named(Name::Dictionary(DictionaryFieldName::Values), parse_quote!(*mut #value));

                let bindings = Depunctuated::from_iter([
                    BindingPresentableContext::Constructor(
                        ConstructorPresentableContext::Default(Name::Constructor(ffi_as_type.clone()), ffi_as_type.clone()),
                        Punctuated::from_iter([
                            OwnedItemPresentableContext::Named(value_conversion.clone(), false),
                            OwnedItemPresentableContext::Named(count_conversion.clone(), false)
                        ]),
                        IteratorPresentationContext::Curly(Punctuated::from_iter([
                            OwnedItemPresentableContext::DefaultField(count_conversion),
                            OwnedItemPresentableContext::DefaultField(value_conversion),
                        ]))
                    ),
                    BindingPresentableContext::Destructor(ffi_as_type.clone())
                ]).present(&source);
                quote! {
                    #object_presentation
                    #conversion_presentation
                    impl ferment_interfaces::FFIVecConversion for #ffi_as_type {
                        type Value = #target_type;
                        unsafe fn decode(&self) -> Self::Value { #from_value_conversion }
                        unsafe fn encode(obj: Self::Value) -> *mut Self { #to_value_conversion }
                    }
                    #drop_presentation
                    #bindings
                }
            },
            FFIObjectPresentation::Empty => { /* Box<T> */
                quote!()
            },
            FFIObjectPresentation::StaticVTable { name, fq_trait_vtable, methods_compositions } => {
                let (methods_implementations, methods_declarations): (Depunctuated<TokenStream2>, Punctuated<TokenStream2, Comma>) = methods_compositions
                    .iter()
                    .map(|TraitVTableMethodComposition { fn_name, ffi_fn_name, item_type, trait_type, argument_names, name_and_args, output_expression, output_conversions }| {
                        (quote!(#fn_name: #ffi_fn_name), {
                            let input_conversions = quote! {
                                let cast_obj = &(*(obj as *const #item_type));
                                let obj = <#item_type as #trait_type>::#fn_name #argument_names;
                            };
                            quote!(#name_and_args -> #output_expression { #input_conversions #output_conversions})
                        })
                    })
                    .unzip();
                quote! {
                    static #name: #fq_trait_vtable = {
                        #methods_implementations
                        #fq_trait_vtable {
                            #methods_declarations
                        }
                    };
                }
            }
        }.to_tokens(tokens)
    }
}
// # [doc = r" # Safety"]
// # [no_mangle]
// pub unsafe extern "C" fn Status_as_CanRetry_can_retry (obj: * const Status) -> bool {
//     let obj = ferment_interfaces::FFIConversion::ffi_from_const(obj);
//     let result = <crate::transport::transport_request::Status as crate::transport::transport_request::CanRetry>::can_retry(&obj);
//     result
// }
