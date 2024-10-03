use quote::quote;
use crate::composable::FieldComposer;
use crate::composer::SourceFermentable;

impl<SPEC> SourceFermentable<crate::lang::objc::ObjCFermentate> for FieldComposer<crate::lang::objc::ObjCFermentate, SPEC>
    where SPEC: crate::lang::objc::ObjCSpecification {
    fn ferment(&self) -> crate::lang::objc::ObjCFermentate {
        let Self { name, kind, attrs, .. } = self;
        crate::lang::objc::ObjCFermentate::TokenStream(attrs.wrap(quote!((#kind)#name)))
    }
}
