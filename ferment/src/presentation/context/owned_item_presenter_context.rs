use std::fmt::{Debug, Display, Formatter};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::Type;
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::ext::FFIResolveExtended;
use crate::presentation::context::FieldTypePresentableContext;
use crate::presentation::ScopeContextPresentable;


#[derive(Clone)]
pub enum OwnedItemPresentableContext {
    DefaultField(FieldTypeConversion),
    DefaultFieldType(Type, TokenStream2),
    Named(FieldTypeConversion, /*is_public:*/ bool),
    Lambda(TokenStream2, TokenStream2, TokenStream2),
    Conversion(TokenStream2, TokenStream2),
    BindingArg(FieldTypeConversion),
    BindingFieldName(FieldTypeConversion),
    FieldType(FieldTypePresentableContext, TokenStream2),

    Exhaustive(TokenStream2),
}

impl Debug for OwnedItemPresentableContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OwnedItemPresentableContext::DefaultField(ty) =>
                f.write_str(format!("DefaultField({})", ty).as_str()),
            OwnedItemPresentableContext::DefaultFieldType(ty, attrs) =>
                f.write_str(format!("DefaultFieldType({}, {})", quote!(#ty), attrs).as_str()),
            OwnedItemPresentableContext::Named(ty, is_pub) =>
                f.write_str(format!("Named({}, {})", ty, is_pub).as_str()),
            OwnedItemPresentableContext::Lambda(lv, rv, attrs) =>
                f.write_str(format!("Lambda({}, {}, {})", lv, rv, attrs).as_str()),
            OwnedItemPresentableContext::Conversion(conversion, attrs) =>
                f.write_str(format!("Conversion({}, {})", conversion, attrs).as_str()),
            OwnedItemPresentableContext::BindingArg(ty) =>
                f.write_str(format!("BindingArg({})", ty).as_str()),
            OwnedItemPresentableContext::BindingFieldName(ty) =>
                f.write_str(format!("BindingField({})", ty).as_str()),
            OwnedItemPresentableContext::FieldType(ctx, attrs) =>
                f.write_str(format!("FieldType({}, {})", ctx, attrs).as_str()),
            OwnedItemPresentableContext::Exhaustive(attrs) =>
                f.write_str(format!("Exhaustive({})", attrs).as_str()),
        }
    }
}

impl Display for OwnedItemPresentableContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}


impl ScopeContextPresentable for OwnedItemPresentableContext {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> TokenStream2 {
        match self {
            OwnedItemPresentableContext::DefaultField(field_type) => {
                let attrs = field_type.attrs();
                let name = field_type.name();
                quote! {
                    #attrs
                    #name
                }
            },
            OwnedItemPresentableContext::DefaultFieldType(field_type, attrs) => {
                let ty = field_type.ffi_full_dictionary_type_presenter(source);
                quote! {
                    #attrs
                    #ty
                }
            },
            OwnedItemPresentableContext::Named(field_type, is_public) => {
                let attrs = field_type.attrs();
                let name = field_type.name();
                let ty = field_type.ty().ffi_full_dictionary_type_presenter(source);
                let acc = (*is_public).then(|| quote!(pub)).unwrap_or_default();
                quote! {
                    #attrs
                    #acc #name: #ty
                }
            }
            OwnedItemPresentableContext::Lambda(name, value, attrs) =>
                quote! {
                    #attrs
                    #name => #value
                },
            OwnedItemPresentableContext::Conversion(conversion, attrs) => quote! {
                #attrs
                #conversion
            },
            OwnedItemPresentableContext::BindingArg(field_type) => {
                match field_type {
                    FieldTypeConversion::Named(field_name, field_type, attrs) => {
                        let ty = field_type.ffi_full_dictionary_type_presenter(source);
                        quote! {
                            #attrs
                            #field_name: #ty
                        }
                    },
                    FieldTypeConversion::Unnamed(field_name, field_type, attrs) => {
                        let field_name = format_ident!("o_{}", field_name.to_token_stream().to_string());
                        let ty = field_type.ffi_full_dictionary_type_presenter(source);
                        quote! {
                            #attrs
                            #field_name: #ty
                        }
                    }
                }
            },
            OwnedItemPresentableContext::BindingFieldName(field_type) => {
                match field_type {
                    FieldTypeConversion::Named(field_name, _ty, attrs) => {
                        quote!(
                            #attrs
                            #field_name
                        )
                    },
                    FieldTypeConversion::Unnamed(field_name, _ty, attrs) => {
                        let field_name = format_ident!("o_{}", field_name.to_token_stream().to_string());
                        quote!(
                            #attrs
                            #field_name
                        )
                    },
                }
            }
            OwnedItemPresentableContext::FieldType(field_type_context, attrs) => {
                let field = field_type_context.present(source);
                quote! {
                    #attrs
                    #field
                }
            }
            OwnedItemPresentableContext::Exhaustive(attrs) => {
                quote! {
                    #attrs
                    _ => unreachable!("This is unreachable")
                }
            }
        }
    }
}

