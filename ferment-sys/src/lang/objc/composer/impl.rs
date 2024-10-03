use crate::composer::{ImplComposer, SourceFermentable};
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};

impl<SPEC> SourceFermentable<ObjCFermentate> for ImplComposer<ObjCFermentate, SPEC>
    where SPEC: ObjCSpecification {
    fn ferment(&self) -> ObjCFermentate {
        ObjCFermentate::Empty
    }
}