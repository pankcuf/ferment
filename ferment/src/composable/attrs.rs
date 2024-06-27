use proc_macro2::Ident;
use quote::ToTokens;
use syn::Attribute;
use crate::ast::Depunctuated;
use crate::context::ScopeChain;
use crate::presentation::Expansion;

pub struct AttrsComposition {
    pub attrs: Vec<Attribute>,
    #[allow(unused)]
    pub ident: Ident,
    #[allow(unused)]
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
    fn cfg_attributes(&self) -> Vec<Attribute>;
    fn cfg_attributes_or_none(&self) -> Vec<Option<Attribute>> {
        let cfg_attrs = self.cfg_attributes();
        cfg_attrs.iter().map(|attr| Some(attr.clone())).collect()
    }
    fn cfg_attributes_expanded(&self) -> Depunctuated<Expansion> {
        self.cfg_attributes()
            .iter()
            .map(|a| Expansion::TokenStream(a.to_token_stream()))
            .collect()
    }
}

impl CfgAttributes for AttrsComposition {
    fn cfg_attributes(&self) -> Vec<Attribute> {
        self.attrs.cfg_attributes()
    }
}

impl CfgAttributes for Vec<Attribute> {
    fn cfg_attributes(&self) -> Vec<Attribute> {
        self.iter()
            .filter(|attr| attr.path.is_ident("cfg"))
            .cloned()
            .collect()
    }
}
impl CfgAttributes for Vec<Option<Attribute>> {
    fn cfg_attributes(&self) -> Vec<Attribute> {
        self.iter()
            .filter_map(|attr| match attr {
                Some(attr) if attr.path.is_ident("cfg") => Some(attr.clone()),
                _ => None
            })
            .collect()
    }
}