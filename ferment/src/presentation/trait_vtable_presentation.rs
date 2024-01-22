use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

pub enum TraitVTablePresentation {
    Full {
        vtable: TokenStream2,
        export: TokenStream2,
        destructor: TokenStream2,
    }
}

impl ToTokens for TraitVTablePresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            TraitVTablePresentation::Full { vtable, export, destructor } => quote! {
                #vtable
                #export
                #destructor
            }
        }.to_tokens(tokens)
    }
}
