use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::Attribute;
use crate::composer::Depunctuated;
use crate::context::ScopeChain;
use crate::presentation::Expansion;

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

pub trait CfgAttributes {
    fn cfg_attributes(&self) -> Depunctuated<Expansion>;
}

impl CfgAttributes for AttrsComposition {
    fn cfg_attributes(&self) -> Depunctuated<Expansion> {
        self.attrs.cfg_attributes()
    }
}

impl CfgAttributes for Vec<Attribute> {
    fn cfg_attributes(&self) -> Depunctuated<Expansion> {
        // println!("cfg_attributes: {:?}", self);
        let result: Depunctuated<Expansion> = self.iter()
            .filter_map(|attr| attr.path.is_ident("cfg")
                .then(|| Expansion::TokenStream(attr.to_token_stream())))
            .collect();
        // println!("cfg_attributes.2: {}", result.to_token_stream());
        result
    }
}