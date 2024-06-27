use quote::{quote, ToTokens};
use syn::__private::TokenStream2;

// Field Type or Fn Arg
#[derive(Clone, Debug)]
#[allow(unused)]
pub enum ArgPresentation {
    AttributedConversion { attrs: TokenStream2, conversion: TokenStream2 },
    NamedType { attrs: TokenStream2, name: TokenStream2, var: TokenStream2 },
    QualifiedNamedType { attrs: TokenStream2, qualifier: TokenStream2, name: TokenStream2, var: TokenStream2 },
    Lambda { attrs: TokenStream2, l_value: TokenStream2, r_value: TokenStream2 },
    Simple { ty: TokenStream2 },
}

impl ToTokens for ArgPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            ArgPresentation::AttributedConversion { attrs , conversion } => quote! {
                #attrs
                #conversion
            },
            ArgPresentation::NamedType { attrs, name, var } => quote! {
                #attrs
                #name: #var
            },
            ArgPresentation::QualifiedNamedType { attrs, qualifier, name, var } => quote! {
                #attrs
                #qualifier #name: #var
            },
            ArgPresentation::Lambda { attrs, l_value, r_value } => quote! {
                #attrs
                #l_value => #r_value
            },
            ArgPresentation::Simple { ty } => quote! {
                #ty
            }
        }.to_tokens(tokens)
    }
}