use quote::quote;
use syn::{Attribute, Type};
use crate::composable::{CfgAttributes, FieldComposer};
use crate::composer::SourceFermentable;
use crate::kind::FieldTypeKind;
use crate::lang::{FromDictionary, RustSpecification};
use crate::presentation::{DictionaryName, Name, RustFermentate};


impl FieldComposer<RustSpecification> {
    pub fn self_typed(ty: Type, attrs: &Vec<Attribute>) -> Self {
        Self::new(Name::dictionary_name(DictionaryName::Self_), FieldTypeKind::Type(ty), true, attrs.cfg_attributes())
    }
}

impl SourceFermentable<RustFermentate> for FieldComposer<RustSpecification> {
    fn ferment(&self) -> RustFermentate {
        let Self { name, kind, attrs, .. } = self;
        RustFermentate::TokenStream(quote!(#(#attrs)* #name: #kind))
    }
}
