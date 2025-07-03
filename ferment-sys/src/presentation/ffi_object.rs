use quote::{quote, ToTokens};
use proc_macro2::{TokenStream as TokenStream2};
use syn::{Attribute, Path};
use crate::presentation::present_struct;

#[derive(Clone, Debug)]
pub enum FFIObjectPresentation {
    TraitVTable {
        name: Path,
        attrs: Vec<Attribute>,
        fields: TokenStream2
    },
    TraitObject {
        name: Path,
        attrs: Vec<Attribute>,
        fields: TokenStream2
    },
    Full(TokenStream2),
    Empty,
}


impl ToTokens for FFIObjectPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Empty => quote!(),
            Self::Full(presentation) => quote!(#presentation),
            Self::TraitVTable { name, attrs, fields } => {
                present_struct(&name.segments.last().unwrap().ident, attrs, fields)
            },
            Self::TraitObject { name, attrs, fields } => {
                present_struct(&name.segments.last().unwrap().ident, attrs, fields)
            },
        }.to_tokens(tokens)
    }
}
