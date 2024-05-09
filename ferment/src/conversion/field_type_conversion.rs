use std::fmt::{Debug, Display, Formatter};
use syn::Type;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use crate::composer::Depunctuated;
use crate::naming::Name;
use crate::presentation::Expansion;

#[derive(Clone)]
pub enum FieldTypeConversion {
    Named(Name, Type, Depunctuated<Expansion>),
    Unnamed(Name, Type, Depunctuated<Expansion>),
}

impl ToTokens for FieldTypeConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            FieldTypeConversion::Named(field_name, ty, attrs) => quote! {
                #attrs
                #field_name: #ty
            },
            FieldTypeConversion::Unnamed(index, ty, attrs) => quote! {
                #attrs
                #index: #ty
            },
        }.to_tokens(tokens)
    }
}
impl Debug for FieldTypeConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldTypeConversion::Named(name, ty, attrs) =>
                f.write_str(format!("Named({}, {}, {})", name.to_token_stream(), ty.to_token_stream(), attrs.to_token_stream()).as_str()),
            FieldTypeConversion::Unnamed(name, ty, attrs) =>
                f.write_str(format!("Unnamed({}, {}, {})", name.to_token_stream(), ty.to_token_stream(), attrs.to_token_stream()).as_str()),
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
            FieldTypeConversion::Named(_, ty, _) => ty,
            FieldTypeConversion::Unnamed(_, ty, _) => ty,
        }
    }
    pub fn name(&self) -> TokenStream2 {
        match self {
            FieldTypeConversion::Named(field_name, _, _) =>
                quote!(#field_name),
            FieldTypeConversion::Unnamed(field_name, _, _) => {
                quote!(#field_name)
            },
        }
    }
    pub fn attrs(&self) -> TokenStream2 {
        match self {
            FieldTypeConversion::Named(_, _, attrs) =>
                quote!(#attrs),
            FieldTypeConversion::Unnamed(_, _, attrs) => {
                quote!(#attrs)
            },
        }
    }
}
