use quote::quote;
use crate::composable::FieldComposer;
use crate::composer::SourceFermentable;
use crate::lang::RustSpecification;
use crate::presentation::RustFermentate;


impl SourceFermentable<RustFermentate> for FieldComposer<RustSpecification> {
    fn ferment(&self) -> RustFermentate {
        let Self { name, kind, attrs, .. } = self;
        RustFermentate::TokenStream(quote!(#(#attrs)* #name: #kind))
    }
}
