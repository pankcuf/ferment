use proc_macro2::Ident;
use syn::Attribute;
use crate::context::ScopeChain;

pub struct AttrsComposition {
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub scope: ScopeChain,
}

impl AttrsComposition {
    pub fn new(attrs: Vec<Attribute>, ident: Ident, scope: ScopeChain) -> Self {
        Self { attrs, ident, scope }
    }

    pub fn from(attrs: &Vec<Attribute>, ident: &Ident, scope: &ScopeChain) -> Self {
        Self::new(attrs.clone(), ident.clone(), scope.clone())
    }
}
