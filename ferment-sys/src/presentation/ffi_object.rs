use quote::ToTokens;
use proc_macro2::{TokenStream as TokenStream2};
use syn::{Attribute, Path, PathSegment};
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
            Self::Empty => {},
            Self::Full(presentation) =>
                presentation.to_tokens(tokens),
            Self::TraitVTable { name: Path { segments, .. }, attrs, fields } |
            Self::TraitObject { name: Path { segments, .. }, attrs, fields } => if let Some(PathSegment { ident, .. }) = segments.last() {
                present_struct(ident, attrs, fields).to_tokens(tokens)
            },
        }
    }
}
