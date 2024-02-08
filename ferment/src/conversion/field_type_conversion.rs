use std::fmt::{Debug, Display, Formatter};
use syn::Type;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use crate::naming::Name;

// pub enum FieldName {
//
// }

#[derive(Clone)]
pub enum FieldTypeConversion {
    Named(Name, Type),
    Unnamed(Name, Type),
    // Named(TokenStream2, Type),
    // Unnamed(TokenStream2, Type),
    //Itself(Type, TokenStream2),
}

impl ToTokens for FieldTypeConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            FieldTypeConversion::Named(field_name, ty) =>
                quote!(#field_name: #ty),
            FieldTypeConversion::Unnamed(index, ty) =>
                quote!(#index: #ty),
            //FieldTypeConversion::Itself(ty, field_name) => quote!(#field_name: #ty),
        }.to_tokens(tokens)
    }
}
impl Debug for FieldTypeConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldTypeConversion::Named(name, ty) =>
                f.write_str(format!("Named({}, {})", name, ty.to_token_stream()).as_str()),
            FieldTypeConversion::Unnamed(name, ty) =>
                f.write_str(format!("Unnamed({}, {})", name, ty.to_token_stream()).as_str()),
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
            //FieldTypeConversion::Itself(ty, _) => ty
        }
    }
    pub fn name(&self) -> TokenStream2 {
        let result = match self {
            FieldTypeConversion::Named(field_name, _) =>
                field_name.clone(),
            FieldTypeConversion::Unnamed(field_name, _) => {
                field_name.clone()
                // let field_name = format_ident!("o_{}", field_name.to_string());
                // field_name.to_token_stream()
            },
            //FieldTypeConversion::Itself(_, field_name) => field_name.clone()
        };
        // println!("name: {} ---> {}", self, result);
        result.to_token_stream()
    }
    pub fn as_binding_arg_name(&self) -> TokenStream2 {
        let result = match self {
            FieldTypeConversion::Named(field_name, _ty) => {
                // let field_name = format_ident!("o_{}", field_name.to_string());
                quote!(#field_name)
            },
            FieldTypeConversion::Unnamed(field_name, _ty) => {
                let field_name = format_ident!("o_{}", field_name.to_token_stream().to_string());
                quote!(#field_name)
            },
            //FieldTypeConversion::Itself(ty, field_name) => quote!(#field_name: #ty)
        };
        // println!("as_binding_arg_name: {} ---> {}", self, result);
        result
    }

}
