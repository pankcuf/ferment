use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::Ident;
use crate::generic_path_conversion::GenericArgPresentation;
use crate::helper::ffi_constructor_name;
use crate::interface::{create_struct, DEFAULT_DOC_PRESENTER, ffi_from_const, ffi_to_const, interface, obj, package, package_boxed_expression};
use crate::scope::Scope;
use crate::scope_conversion::{ScopeTree, ScopeTreeCompact};

/// Root-level composer chain
pub enum Expansion {
    Empty,
    Callback {
        comment: DocPresentation,
        ffi_presentation: FFIObjectPresentation,
    },
    Function {
        comment: DocPresentation,
        ffi_presentation: FFIObjectPresentation,
    },
    Full {
        comment: DocPresentation,
        ffi_presentation: FFIObjectPresentation,
        conversion: ConversionInterfacePresentation,
        drop: DropInterfacePresentation,
        bindings: Vec<BindingPresentation>,
        traits: Vec<TraitVTablePresentation>,
    },
    Root {
        tree: ScopeTree,
    },
    Mod {
        directives: TokenStream2,
        name: TokenStream2,
        imports: Vec<Scope>,
        conversions: Vec<TokenStream2>
    },
    Use {
        comment: DocPresentation,
    },
    Trait {
        comment: DocPresentation,
        vtable: FFIObjectPresentation,
        trait_object: FFIObjectPresentation,
    }
}


pub enum DocPresentation {
    Empty,
    Default(TokenStream2),
    Safety(TokenStream2),
}

impl From<ScopeTreeCompact> for Expansion {
    fn from(value: ScopeTreeCompact) -> Self {
        Expansion::Root { tree: value.into() }
    }
}

pub enum BindingPresentation {
    Constructor {
        ffi_ident: Ident,
        ctor_arguments: Vec<TokenStream2>,
        body_presentation: TokenStream2
    },
    EnumVariantConstructor {
        ffi_ident: TokenStream2,
        ffi_variant_ident: Ident,
        ffi_variant_path: TokenStream2,
        ctor_arguments: Vec<TokenStream2>,
        body_presentation: TokenStream2
    },
    Destructor {
        ffi_name: TokenStream2,
        destructor_ident: TokenStream2
    },
}

pub enum FFIObjectPresentation {
    // Empty,
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
        fields: Vec<TokenStream2>
    },
    TraitObject {
        name: TokenStream2,
        vtable_name: TokenStream2,
    },
    Full(TokenStream2),
    Result {
        target_type: TokenStream2,
        ffi_type: TokenStream2,

        ok_presentation: GenericArgPresentation,
        error_presentation: GenericArgPresentation,
    },
    Map {
        target_type: TokenStream2,
        ffi_type: TokenStream2,

        key_presentation: GenericArgPresentation,
        value_presentation: GenericArgPresentation,
    },
    Vec {
        target_type: TokenStream2,
        ffi_type: TokenStream2,
        value_presentation: GenericArgPresentation,
    },
    // Generic {
    //     target_type: TokenStream2,
    //     ffi_type: TokenStream2,
    //     arg_presentations: Vec<TokenStream2>
    // }
}

pub enum ConversionInterfacePresentation {
    Interface {
        ffi_type: TokenStream2,
        target_type: TokenStream2,
        from_presentation: FromConversionPresentation,
        to_presentation: ToConversionPresentation,
        destroy_presentation: TokenStream2
    },
    Empty
}

pub enum TraitVTablePresentation {
    Full {
        vtable: TokenStream2,
        export: TokenStream2,
        destructor: TokenStream2,
    }
}

pub enum DropInterfacePresentation {
    Empty,
    Full(TokenStream2, TokenStream2)
}

pub enum FromConversionPresentation {
    Enum(Vec<TokenStream2>),
    Struct(TokenStream2),
    Vec,
    Map(TokenStream2, TokenStream2),
    Result(TokenStream2, TokenStream2),
}

pub enum ToConversionPresentation {
    Enum(Vec<TokenStream2>),
    Struct(TokenStream2),
    Vec,
    Map(TokenStream2, TokenStream2),
    Result(TokenStream2, TokenStream2)
}

impl ToTokens for Expansion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let presentations = match self {
            Self::Empty | Self::Use { comment: _ } => vec![],
            Self::Callback { comment, ffi_presentation } =>
                vec![comment.to_token_stream(), ffi_presentation.to_token_stream()],
            Self::Function { comment, ffi_presentation } =>
                vec![comment.to_token_stream(), ffi_presentation.to_token_stream()],
            Self::Full { comment, ffi_presentation, conversion, drop, bindings, traits } => {
                let mut full = vec![comment.to_token_stream(), ffi_presentation.to_token_stream(), conversion.to_token_stream(), drop.to_token_stream()];
                full.extend(bindings.iter().map(BindingPresentation::to_token_stream));
                full.extend(traits.iter().map(TraitVTablePresentation::to_token_stream));
                full
            },
            Self::Mod { directives, name, imports: _, conversions } =>
                vec![
                    quote! {
                        #directives
                        pub mod #name {
                            //#(use #imports;)*
                            #(#conversions)*
                        }
                    }
                ],
            Self::Trait { comment, vtable, trait_object } =>
                vec![comment.to_token_stream(), vtable.to_token_stream(), trait_object.to_token_stream()],
            Self::Root { tree } =>
                vec![tree.to_token_stream()]
        };
        let expanded = quote!(#(#presentations)*);
        // println!("{}", expanded);
        expanded.to_tokens(tokens)
    }
}

impl ToTokens for DocPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Empty => quote!(),
            Self::Default(target_name) => DEFAULT_DOC_PRESENTER(quote!(#target_name)),
            Self::Safety(target_name) => {
                let doc = DEFAULT_DOC_PRESENTER(quote!(#target_name));
                quote! {
                    #doc
                    /// # Safety
                }
            }
        }.to_tokens(tokens)
    }
}
impl ToTokens for BindingPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Constructor { ffi_ident, ctor_arguments, body_presentation} => {
                // quote!()
                let ffi_name = ffi_constructor_name(ffi_ident);
                quote! {
                    /// # Safety
                    #[allow(non_snake_case)]
                    #[no_mangle]
                    pub unsafe extern "C" fn #ffi_name(#(#ctor_arguments),*) -> *mut #ffi_ident {
                        ferment_interfaces::boxed(#ffi_ident #body_presentation)
                    }
                }
            },
            Self::EnumVariantConstructor { ffi_ident, ffi_variant_ident, ffi_variant_path, ctor_arguments, body_presentation} => {
                let ffi_name = ffi_constructor_name(ffi_variant_ident);
                quote! {
                    /// # Safety
                    #[allow(non_snake_case)]
                    #[no_mangle]
                    pub unsafe extern "C" fn #ffi_name(#(#ctor_arguments),*) -> *mut #ffi_ident {
                        ferment_interfaces::boxed(#ffi_variant_path #body_presentation)
                    }
                }
            },
            Self::Destructor { ffi_name, destructor_ident } => {
                quote! {
                    /// # Safety
                    #[allow(non_snake_case)]
                    #[no_mangle]
                    pub unsafe extern "C" fn #destructor_ident(ffi: *mut #ffi_name) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
            }
        }.to_tokens(tokens)
     }
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
                        let obj = rt.block_on(async {
                            let obj = #input_conversions .await;
                            obj
                        });
                        #output_conversions
                    }
                }
            },
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
            Self::Result { target_type, ffi_type, ok_presentation, error_presentation} => {
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
                };
                let drop_presentation = DropInterfacePresentation::Full(quote!(#ffi_type), quote!(#(#drop_code)*));
                quote! {
                    #object_presentation
                    #conversion_presentation
                    #drop_presentation
                }
            },
            Self::Map { target_type, ffi_type, key_presentation, value_presentation} => {
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
                };
                let drop_presentation = DropInterfacePresentation::Full(quote!(#ffi_type), quote!(#(#drop_code)*));
                quote! {
                    #object_presentation
                    #conversion_presentation
                    #drop_presentation
                }
            },
            FFIObjectPresentation::Vec { target_type, ffi_type, value_presentation } => {
                let GenericArgPresentation { ty: value, from_conversion: from_value_conversion, to_conversion: to_value_conversion, destructor: value_destructor } = value_presentation;
                let drop_code = [value_destructor];

                let conversion_presentation = ConversionInterfacePresentation::Interface {
                    ffi_type: quote!(#ffi_type),
                    target_type: quote!(#target_type),
                    from_presentation: FromConversionPresentation::Vec,
                    to_presentation: ToConversionPresentation::Vec,
                    destroy_presentation: quote!(ferment_interfaces::unbox_any(ffi);),
                };
                let object_presentation = create_struct(quote!(#ffi_type), quote!({
                        pub count: usize,
                        pub values: *mut #value,
                    }));
                let drop_presentation = DropInterfacePresentation::Full(ffi_type.to_token_stream(), quote!(#(#drop_code)*));
                quote! {
                    #object_presentation
                    #conversion_presentation
                    impl ferment_interfaces::FFIVecConversion for #ffi_type {
                        type Value = #target_type;
                        unsafe fn decode(&self) -> Self::Value { #from_value_conversion }
                        unsafe fn encode(obj: Self::Value) -> *mut Self { #to_value_conversion }
                    }
                    #drop_presentation
                }
            },
            // FFIObjectPresentation::Generic { .. } => {}
        }.to_tokens(tokens)
    }
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

impl ToTokens for ToConversionPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            ToConversionPresentation::Enum(conversions) => {
                package_boxed_expression(quote!(match obj { #(#conversions,)* }))
            },
            ToConversionPresentation::Struct(conversion) => {
                quote! {
                    #conversion
                }
            },
            ToConversionPresentation::Vec =>
                quote!(ferment_interfaces::FFIVecConversion::encode(obj)),
            ToConversionPresentation::Map(to_key_conversion, to_value_conversion) =>
                quote!(ferment_interfaces::boxed(Self { count: obj.len(), keys: #to_key_conversion, values: #to_value_conversion  })),
            ToConversionPresentation::Result(to_ok_conversion, to_error_conversion) => quote! {
                let (ok, error) = match obj {
                    Ok(o) => (#to_ok_conversion, std::ptr::null_mut()),
                    Err(o) => (std::ptr::null_mut(), #to_error_conversion)
                };
                ferment_interfaces::boxed(Self { ok, error })
            }
        }.to_tokens(tokens)
    }
}

impl ToTokens for ConversionInterfacePresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Empty => quote!(),
            Self::Interface { ffi_type: ffi_name, target_type: target_name, from_presentation, to_presentation, destroy_presentation} => {
                let package = package();
                let interface = interface();
                let obj = obj();
                // let ffi_from = ffi_from();
                let ffi_from_const = ffi_from_const();
                // let ffi_to = ffi_to();
                let ffi_to_const = ffi_to_const();
                // let ffi_from_opt = ffi_from_opt();
                // let ffi_to_opt = ffi_to_opt();
                quote! {
                    impl #package::#interface<#target_name> for #ffi_name {
                        unsafe fn #ffi_from_const(ffi: *const #ffi_name) -> #target_name { #from_presentation }
                        unsafe fn #ffi_to_const(#obj: #target_name) -> *const #ffi_name { #to_presentation }
                        // unsafe fn #ffi_from(ffi: *mut #ffi_name) -> #target_name { #ffi_from_conversion }
                        // unsafe fn #ffi_to(#obj: #target_name) -> *mut #ffi_name { #ffi_to_conversion }
                        // unsafe fn #ffi_from_opt(ffi: *mut #ffi_name) -> Option<#target_name> {
                        //     (!#ffi.is_null()).then_some(<Self as #package::#interface<#target_name>>::#ffi_from(ffi))
                        // }
                        // unsafe fn #ffi_to_opt(#obj: Option<#target_name>) -> *mut #ffi_name {
                        //     #obj.map_or(std::ptr::null_mut(), |o| <Self as #package::#interface<#target_name>>::#ffi_to(o))
                        // }
                        unsafe fn destroy(ffi: *mut #ffi_name) { #destroy_presentation; }
                    }

                }
            },
        }.to_tokens(tokens)
    }
}

impl ToTokens for DropInterfacePresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Empty => quote!(),
            Self::Full(name, code) =>
                quote!(impl Drop for #name { fn drop(&mut self) { unsafe { #code } } })
        }.to_tokens(tokens)
    }
}

impl ToTokens for TraitVTablePresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            TraitVTablePresentation::Full { vtable, export, destructor } => quote! {
                #vtable
                #export
                #destructor
            }
        }.to_tokens(tokens)
    }
}
