use syn::Type;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

#[derive(Clone)]
pub enum FieldTypeConversion {
    Named(Type, TokenStream2),
    Unnamed(Type, TokenStream2),
    //Itself(Type, TokenStream2),
}

impl ToTokens for FieldTypeConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            FieldTypeConversion::Named(ty, field_name) => quote!(#field_name: #ty),
            FieldTypeConversion::Unnamed(ty, index) => quote!(#index: #ty),
            //FieldTypeConversion::Itself(ty, field_name) => quote!(#field_name: #ty),
        }.to_tokens(tokens)
    }
}

impl FieldTypeConversion {
    pub fn ty(&self) -> &Type {
        match self {
            FieldTypeConversion::Named(ty, _) => ty,
            FieldTypeConversion::Unnamed(ty, _) => ty,
            //FieldTypeConversion::Itself(ty, _) => ty
        }
    }
    pub fn name(&self) -> TokenStream2 {
        match self {
            FieldTypeConversion::Named(_, field_name) => field_name.clone(),
            FieldTypeConversion::Unnamed(_, field_name) => field_name.clone(),
            //FieldTypeConversion::Itself(_, field_name) => field_name.clone()
        }

    }
}
