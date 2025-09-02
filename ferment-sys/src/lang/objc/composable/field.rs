use quote::quote;
use crate::composable::FieldComposer;
use crate::composer::SourceFermentable;
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};

impl SourceFermentable<ObjCFermentate> for FieldComposer<ObjCSpecification> {
    fn ferment(&self) -> ObjCFermentate {
        let Self { name, kind, attrs, .. } = self;
        ObjCFermentate::TokenStream(attrs.wrap(quote!((#kind)#name)))
    }
}
