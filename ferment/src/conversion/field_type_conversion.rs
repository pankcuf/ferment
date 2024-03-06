use std::fmt::{Debug, Display, Formatter};
use syn::Type;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use crate::naming::Name;

#[derive(Clone)]
pub enum FieldTypeConversion {
    Named(Name, Type),
    Unnamed(Name, Type),
}

impl ToTokens for FieldTypeConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            FieldTypeConversion::Named(field_name, ty) =>
                quote!(#field_name: #ty),
            FieldTypeConversion::Unnamed(index, ty) =>
                quote!(#index: #ty),
        }.to_tokens(tokens)
    }
}
impl Debug for FieldTypeConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldTypeConversion::Named(name, ty) =>
                f.write_str(format!("Named({:?}, {})", name, ty.to_token_stream()).as_str()),
            FieldTypeConversion::Unnamed(name, ty) =>
                f.write_str(format!("Unnamed({:?}, {})", name, ty.to_token_stream()).as_str()),
        }
    }
}

impl Display for FieldTypeConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl FieldTypeConversion {
    pub fn ty(&self) -> &Type {
        match self {
            FieldTypeConversion::Named(_, ty) => ty,
            FieldTypeConversion::Unnamed(_, ty) => ty,
        }
    }
    pub fn name(&self) -> TokenStream2 {
        match self {
            FieldTypeConversion::Named(field_name, _) =>
                quote!(#field_name),
            FieldTypeConversion::Unnamed(field_name, _) => {
                quote!(#field_name)
            },
        }
    }
}
