use proc_macro2::Ident;
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::Path;

pub enum GenericBoundKey {
    Ident(Ident),
    Path(Path)
}

impl GenericBoundKey {
    pub fn ident(ident: &Ident) -> Self {
        Self::Ident(ident.clone())
    }
    pub fn path(path: &Path) -> Self {
        Self::Path(path.clone())
    }
}

impl ToTokens for GenericBoundKey {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            GenericBoundKey::Ident(ident) => ident.to_tokens(tokens),
            GenericBoundKey::Path(path) => path.to_tokens(tokens),
        }
    }
}

