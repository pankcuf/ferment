use std::fmt::{Debug, Display, Formatter};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::Type;
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::presentation::context::FieldTypePresentableContext;
use crate::presentation::ScopeContextPresentable;


#[derive(Clone)]
pub enum OwnedItemPresentableContext {
    DefaultField(FieldTypeConversion),
    DefaultFieldType(Type),
    Named(FieldTypeConversion, /*is_public:*/ bool),
    Lambda(TokenStream2, TokenStream2),
    Conversion(TokenStream2),
    BindingArg(FieldTypeConversion),
    BindingFieldName(FieldTypeConversion),
    FieldType(FieldTypePresentableContext),

}

impl Debug for OwnedItemPresentableContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OwnedItemPresentableContext::DefaultField(ty) =>
                f.write_str(format!("DefaultField({})", ty).as_str()),
            OwnedItemPresentableContext::DefaultFieldType(ty) =>
                f.write_str(format!("DefaultFieldType({})", quote!(#ty)).as_str()),
            OwnedItemPresentableContext::Named(ty, is_pub) =>
                f.write_str(format!("Named({}, {})", ty, is_pub).as_str()),
            OwnedItemPresentableContext::Lambda(lv, rv) =>
                f.write_str(format!("Lambda({}, {})", lv, rv).as_str()),
            OwnedItemPresentableContext::Conversion(conversion) =>
                f.write_str(format!("Conversion({})", conversion).as_str()),
            OwnedItemPresentableContext::BindingArg(ty) =>
                f.write_str(format!("BindingArg({})", ty).as_str()),
            OwnedItemPresentableContext::BindingFieldName(ty) =>
                f.write_str(format!("BindingField({})", ty).as_str()),
            OwnedItemPresentableContext::FieldType(ctx) =>
                f.write_str(format!("FieldType({:?})", ctx).as_str()),
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
                field_type.name()
            },
            OwnedItemPresentableContext::DefaultFieldType(field_type) =>
                source.ffi_full_dictionary_type_presenter(field_type)
                    .to_token_stream(),
            OwnedItemPresentableContext::Named(field_type, is_public) => {
                let name = field_type.name();
                let ty = source.ffi_full_dictionary_type_presenter(field_type.ty());
                if *is_public {
                    quote!(pub #name: #ty)
                } else {
                    quote!(#name: #ty)
                }
            }
            OwnedItemPresentableContext::Lambda(name, value) =>
                quote!(#name => #value),
            OwnedItemPresentableContext::Conversion(conversion) =>
                quote!(#conversion),
            OwnedItemPresentableContext::BindingArg(field_type) => {
                match field_type {
                    FieldTypeConversion::Named(field_name, field_type) => {
                        let ty = source.ffi_full_dictionary_type_presenter(field_type);
                        quote!(#field_name: #ty)
                    },
                    FieldTypeConversion::Unnamed(field_name, field_type) => {
                        let field_name = format_ident!("o_{}", field_name.to_token_stream().to_string());
                        let ty = source.ffi_full_dictionary_type_presenter(field_type);
                        quote!(#field_name: #ty)
                    }
                }
            },
            OwnedItemPresentableContext::BindingFieldName(field_type) => {
                match field_type {
                    FieldTypeConversion::Named(field_name, _ty) => {
                        quote!(#field_name)
                    },
                    FieldTypeConversion::Unnamed(field_name, _ty) => {
                        let field_name = format_ident!("o_{}", field_name.to_token_stream().to_string());
                        quote!(#field_name)
                    },
                }
            }
            OwnedItemPresentableContext::FieldType(field_type_context) => {
                field_type_context.present(source)
            }
        }
    }
}

