use syn::{Attribute, Generics};

pub struct AttrsModel {
    pub attrs: Vec<Attribute>,
    // #[allow(unused)]
    // pub ident: Ident,
    // #[allow(unused)]
    // pub scope: ScopeChain,
}
pub struct GenModel {
    pub generics: Option<Generics>,
    // #[allow(unused)]
    // pub ident: Ident,
    // #[allow(unused)]
    // pub scope: ScopeChain,
}

impl Default for GenModel {
    fn default() -> Self {
        Self { generics: None }
    }
}

impl AttrsModel {
    pub fn new(attrs: Vec<Attribute>) -> Self {
        Self { attrs }
    }

    pub fn from(attrs: &Vec<Attribute>) -> Self {
        Self::new(attrs.clone())
    }
}
impl GenModel {
    pub fn new(generics: Option<Generics>) -> Self {
        Self { generics }
    }

    pub fn from(generics: &Option<Generics>) -> Self {
        Self::new(generics.clone())
    }
}

pub trait CfgAttributes {
    fn cfg_attributes(&self) -> Vec<Attribute>;
    fn cfg_attributes_or_none(&self) -> Vec<Option<Attribute>> {
        let cfg_attrs = self.cfg_attributes();
        cfg_attrs.iter().map(|attr| Some(attr.clone())).collect()
    }
    // #[allow(unused)]
    // fn cfg_attributes_expanded(&self) -> Directives {
    //     self.cfg_attributes()
    //         .iter()
    //         .map(|a| RustFermentate::TokenStream(a.to_token_stream()))
    //         .collect()
    // }
}

impl CfgAttributes for AttrsModel {
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