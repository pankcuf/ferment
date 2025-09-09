use syn::{Attribute, Generics, Lifetime};

pub struct AttrsModel {
    pub attrs: Vec<Attribute>,
}
impl AttrsModel {
    pub fn new(attrs: Vec<Attribute>) -> Self {
        Self { attrs }
    }

    pub fn from(attrs: &[Attribute]) -> Self {
        Self::new(attrs.to_owned())
    }
}

#[derive(Default)]
pub struct GenModel {
    pub generics: Option<Generics>,
}

impl From<&Option<Generics>> for GenModel {
    fn from(value: &Option<Generics>) -> Self {
        Self::new(value.clone())
    }
}
impl From<&Generics> for GenModel {
    fn from(value: &Generics) -> Self {
        Self::new(Some(value.clone()))
    }
}
impl GenModel {
    pub fn new(generics: Option<Generics>) -> Self {
        Self { generics }
    }
}

#[derive(Default)]
pub struct LifetimesModel {
    pub lifetimes: Vec<Lifetime>,
}

impl LifetimesModel {
    pub fn new(lifetimes: Vec<Lifetime>) -> Self {
        Self { lifetimes }
    }
}



pub trait CfgAttributes {
    fn cfg_attributes(&self) -> Vec<Attribute>;
    fn cfg_attributes_or_none(&self) -> Vec<Option<Attribute>> {
        self.cfg_attributes().into_iter().map(Some).collect()
    }
}

impl CfgAttributes for AttrsModel {
    fn cfg_attributes(&self) -> Vec<Attribute> {
        self.attrs.cfg_attributes()
    }
}

impl CfgAttributes for Vec<Attribute> {
    fn cfg_attributes(&self) -> Vec<Attribute> {
        self.iter()
            .filter(|attr| attr.path().is_ident("cfg"))
            .cloned()
            .collect()
    }
}
impl CfgAttributes for Vec<Option<Attribute>> {
    fn cfg_attributes(&self) -> Vec<Attribute> {
        self.iter()
            .filter_map(|attr| match attr {
                Some(attr) if attr.path().is_ident("cfg") => Some(attr.clone()),
                _ => None
            })
            .collect()
    }
}