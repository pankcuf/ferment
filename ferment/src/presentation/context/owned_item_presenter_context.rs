use std::fmt::{Debug, Display, Formatter};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::presentation::ScopeContextPresentable;


#[derive(Clone)]
pub enum OwnedItemPresenterContext {
    DefaultField(FieldTypeConversion),
    DefaultFieldType(FieldTypeConversion),
    Named(FieldTypeConversion, /*is_public:*/ bool),
    Lambda(TokenStream2, TokenStream2),
    Conversion(TokenStream2),
    BindingArg(FieldTypeConversion),
    BindingField(FieldTypeConversion),
    BindingGetter(FieldTypeConversion),
    BindingSetter(FieldTypeConversion),
}

impl Debug for OwnedItemPresenterContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OwnedItemPresenterContext::DefaultField(ty) =>
                f.write_str(format!("DefaultField({})", ty).as_str()),
            OwnedItemPresenterContext::DefaultFieldType(ty) =>
                f.write_str(format!("DefaultFieldType({})", ty).as_str()),
            OwnedItemPresenterContext::Named(ty, is_pub) =>
                f.write_str(format!("Named({}, {})", ty, is_pub).as_str()),
            OwnedItemPresenterContext::Lambda(lv, rv) =>
                f.write_str(format!("Lambda({}, {})", lv, rv).as_str()),
            OwnedItemPresenterContext::Conversion(conversion) =>
                f.write_str(format!("Conversion({})", conversion).as_str()),
            OwnedItemPresenterContext::BindingArg(ty) =>
                f.write_str(format!("BindingArg({})", ty).as_str()),
            OwnedItemPresenterContext::BindingField(ty) =>
                f.write_str(format!("BindingField({})", ty).as_str()),
            OwnedItemPresenterContext::BindingGetter(ty) =>
                f.write_str(format!("BindingGetter({})", ty).as_str()),
            OwnedItemPresenterContext::BindingSetter(ty) =>
                f.write_str(format!("BindingSetter({})", ty).as_str())
        }
    }
}

impl Display for OwnedItemPresenterContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}


impl ScopeContextPresentable for OwnedItemPresenterContext {
    type Presentation = TokenStream2;

    fn present(&self, context: &ScopeContext) -> TokenStream2 {
        match self {
            OwnedItemPresenterContext::DefaultField(field_type) => {
                field_type.name()
            },
            OwnedItemPresenterContext::DefaultFieldType(field_type) =>
                context.ffi_full_dictionary_field_type_presenter(field_type.ty())
                    .to_token_stream(),
            OwnedItemPresenterContext::Named(field_type, is_public) => {
                let name = field_type.name();
                let ty = context.ffi_full_dictionary_field_type_presenter(field_type.ty());
                if *is_public {
                    quote!(pub #name: #ty)
                } else {
                    quote!(#name: #ty)
                }
            }
            OwnedItemPresenterContext::Lambda(name, value) =>
                quote!(#name => #value),
            OwnedItemPresenterContext::Conversion(conversion) =>
                quote!(#conversion),
            OwnedItemPresenterContext::BindingArg(field_type) => {
                let name = field_type.as_binding_arg_name();
                let ty = context.ffi_full_dictionary_field_type_presenter(field_type.ty());
                quote!(#name: #ty)

            },
            OwnedItemPresenterContext::BindingField(field_type) => {
                field_type.as_binding_arg_name().to_token_stream()
            }
            OwnedItemPresenterContext::BindingGetter(field_type) => {
                field_type.name()
            }
            OwnedItemPresenterContext::BindingSetter(field_type) => {
                let ty = context.ffi_full_dictionary_field_type_presenter(field_type.ty());
                ty.to_token_stream()
            }
        }
    }
}

