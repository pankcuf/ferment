use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::Ident;
use crate::helper::ffi_constructor_name;
use crate::interface::{DEFAULT_DOC_PRESENTER, ffi_from_const, ffi_to_const, interface, obj, package};
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
        constructor: ConversionInterfacePresentation,
        destructor: ConversionInterfacePresentation,
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

pub enum FFIObjectPresentation {
    // Empty,
    Callback {
        name: TokenStream2,
        arguments: Vec<TokenStream2>,
        output_expression: TokenStream2,
    },
    Function {
        name_and_arguments: TokenStream2,
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
        ok_type: TokenStream2,
        error_type: TokenStream2,
        from_conversion: TokenStream2,
        to_conversion: TokenStream2,
        drop_presentation: TokenStream2,
    },
}

pub enum ConversionInterfacePresentation {
    Interface {
        ffi_type: TokenStream2,
        target_type: TokenStream2,
        from_presentation: FromConversionPresentation,
        to_presentation: ToConversionPresentation,
        destroy_presentation: TokenStream2
    },
    Constructor {
        ffi_ident: Ident,
        // constructor_ident: TokenStream2,
        ctor_arguments: Vec<TokenStream2>,
        body_presentation: TokenStream2
    },
    Destructor {
        ffi_name: TokenStream2,
        destructor_ident: TokenStream2
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
    Struct(TokenStream2)
}

pub enum ToConversionPresentation {
    Enum(Vec<TokenStream2>),
    Struct(TokenStream2)
}

impl ToTokens for Expansion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let presentations = match self {
            Self::Empty | Self::Use { comment: _ } => vec![],
            Self::Callback { comment, ffi_presentation } =>
                vec![comment.to_token_stream(), ffi_presentation.to_token_stream()],
            Self::Function { comment, ffi_presentation } =>
                vec![comment.to_token_stream(), ffi_presentation.to_token_stream()],
            Self::Full { comment, ffi_presentation, conversion, drop, constructor, destructor, traits } => {
                let mut full = vec![comment.to_token_stream(), ffi_presentation.to_token_stream(), conversion.to_token_stream(), drop.to_token_stream(), constructor.to_token_stream(), destructor.to_token_stream()];
                full.extend(traits.into_iter().map(|trait_presentation| trait_presentation.to_token_stream()));
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

impl ToTokens for FFIObjectPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Callback { name, arguments, output_expression} =>
                quote! {
                    #[allow(non_camel_case_types)]
                    pub type #name = unsafe extern "C" fn(#(#arguments),*) #output_expression;
                },
            Self::Function { name_and_arguments, input_conversions, output_expression, output_conversions, } => {
                let macros = quote!(#[no_mangle]);
                let signature = quote!(pub unsafe extern "C" fn #name_and_arguments -> #output_expression);
                let body = quote!({ let obj = #input_conversions; #output_conversions });
                quote! {
                    #macros
                    #signature
                    #body
                }
            },
            Self::Full(presentation) => quote!(#presentation),
            Self::TraitVTable { name, fields } => quote! {
                #[repr(C)]
                #[derive(Clone)]
                #[allow(non_camel_case_types)]
                pub struct #name {
                    #(#fields,)*
                }
            },
            Self::TraitObject { name, vtable_name } => {
                quote! {
                    #[repr(C)]
                    #[derive(Clone)]
                    #[allow(non_camel_case_types)]
                    pub struct #name {
                        pub object: *const (),
                        pub vtable: *const #vtable_name
                    }
                }
            },
            Self::Result { target_type, ffi_type, ok_type, error_type, from_conversion, to_conversion, drop_presentation} => {
                quote! {
                    #[repr(C)]
                    #[derive(Clone)]
                    #[allow(non_camel_case_types)]
                    pub struct #ffi_type {
                        pub ok: *mut #ok_type,
                        pub error: *mut #error_type,
                    }
                     impl ferment_interfaces::FFIConversion<#target_type> for #ffi_type {
                        unsafe fn ffi_from_const(ffi: *const #ffi_type) -> #target_type {
                            #from_conversion
                        }
                        unsafe fn ffi_to_const(obj: #target_type) -> *const #ffi_type {
                            #to_conversion
                        }
                        unsafe fn destroy(ffi: *mut #ffi_type) {
                            ferment_interfaces::unbox_any(ffi);
                        }
                    }
                    #drop_presentation
                }
            }
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
            }
        }.to_tokens(tokens)
    }
}

impl ToTokens for ToConversionPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            ToConversionPresentation::Enum(conversions) => {
                quote! {
                    ferment_interfaces::boxed(match obj {
                        #(#conversions,)*
                    })
                }
            },
            ToConversionPresentation::Struct(conversion) => {
                quote! {
                    #conversion
                }
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
