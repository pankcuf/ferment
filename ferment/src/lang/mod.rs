pub(crate) mod rust;

#[cfg(feature = "objc")]
pub(crate) mod objc;

#[cfg(feature = "java")]
pub(crate) mod java;


use syn::{Attribute, Generics};
use crate::error;
use crate::lang::objc::composers::AttrWrapper;
use crate::tree::CrateTree;


pub trait CrateTreeConsumer {
    fn generate(&self, crate_tree: &CrateTree) -> Result<(), error::Error>;
}

pub trait LangAttrSpecification<T: Clone>: Clone + Default {

    fn from_attrs(attrs: Vec<Attribute>) -> Self;
}
pub trait LangGenSpecification<T: Clone>: Clone + Default {

    fn from_generics(generics: Option<Generics>) -> Self;
}

impl<T> LangAttrSpecification<T> for Vec<Attribute> where T: Clone {
    fn from_attrs(attrs: Vec<Attribute>) -> Self {
        attrs
    }
}
impl<T> LangGenSpecification<T> for Option<Generics> where T: Clone {
    fn from_generics(generics: Option<Generics>) -> Self {
        generics
    }
}
impl<T> LangAttrSpecification<T> for AttrWrapper where T: Clone {
    fn from_attrs(attrs: Vec<Attribute>) -> Self {
        AttrWrapper::from(attrs)
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Lang {
    #[cfg(feature = "objc")]
    ObjC(objc::Config),
    #[cfg(feature = "java")]
    Java(java::Config)
}

impl CrateTreeConsumer for Lang {
    fn generate(&self, crate_tree: &CrateTree) -> Result<(), error::Error> {
        match self {
            #[cfg(feature = "objc")]
            Lang::ObjC(config) =>
                config.generate(crate_tree),
            #[cfg(feature = "java")]
            Lang::Java(config) =>
                config.generate(crate_tree),
            #[cfg(all(not(feature = "objc"), not(feature = "java")))]
            _ => Ok(())
        }
    }
}

// pub struct ScopeTreeFermentate {
//     pub lang: Lang,
//     pub tree: ScopeTree
// }