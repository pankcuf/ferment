use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
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
}

impl ScopeContextPresentable for OwnedItemPresenterContext {
    type Presentation = TokenStream2;

    fn present(&self, context: &ScopeContext) -> TokenStream2 {
        match self {
            OwnedItemPresenterContext::DefaultField(field_type) =>
                field_type.name(),
            OwnedItemPresenterContext::DefaultFieldType(field_type) =>
                context.ffi_full_dictionary_field_type_presenter(field_type.ty()),
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
            // FieldTypedPresenterContext::DefaultType(ty) => ty,
            OwnedItemPresenterContext::Conversion(conversion) => conversion.clone(),
            OwnedItemPresenterContext::BindingArg(field_type) => {
                let name = field_type.as_binding_arg_name();
                let ty = context.ffi_full_dictionary_field_type_presenter(field_type.ty());
                quote!(#name: #ty)

            },
        }
    }
}

