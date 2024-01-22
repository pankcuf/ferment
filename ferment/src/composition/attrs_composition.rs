use proc_macro2::Ident;
use syn::Attribute;
use crate::holder::PathHolder;

pub struct AttrsComposition {
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub scope: PathHolder,
}