use crate::composer::{SigComposer, SourceFermentable};
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};

impl<SPEC> SourceFermentable<ObjCFermentate> for SigComposer<ObjCFermentate, SPEC>
    where SPEC: ObjCSpecification {
    fn ferment(&self) -> ObjCFermentate {
        ObjCFermentate::Empty
    }
}