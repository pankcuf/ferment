use syn::Type;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};

// pub enum FieldName {
//
// }

#[derive(Clone)]
pub enum FieldTypeConversion {
    Named(Type, TokenStream2),
    Unnamed(Type, TokenStream2),
    //Itself(Type, TokenStream2),
}

impl ToTokens for FieldTypeConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            FieldTypeConversion::Named(ty, field_name) =>
                quote!(#field_name: #ty),
            FieldTypeConversion::Unnamed(ty, index) =>
                quote!(#index: #ty),
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
            FieldTypeConversion::Unnamed(_, field_name) => {
                let field_name = format_ident!("o_{}", field_name.to_string());
                field_name.to_token_stream()
            },
            //FieldTypeConversion::Itself(_, field_name) => field_name.clone()
        }

    }
    pub fn as_binding_arg_name(&self) -> TokenStream2 {

        match self {
            FieldTypeConversion::Named(_ty, field_name) => {
                // let field_name = format_ident!("o_{}", field_name.to_string());
                quote!(#field_name)
            },
            FieldTypeConversion::Unnamed(_ty, field_name) => {
                let field_name = format_ident!("o_{}", field_name.to_string());
                quote!(#field_name)
            },
            //FieldTypeConversion::Itself(ty, field_name) => quote!(#field_name: #ty)
        }
    }

}
