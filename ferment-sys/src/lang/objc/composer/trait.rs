use crate::composer::{SourceFermentable, TraitComposer};
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};

impl<SPEC> SourceFermentable<ObjCFermentate> for TraitComposer<ObjCFermentate, SPEC>
    where SPEC: ObjCSpecification {
    fn ferment(&self) -> ObjCFermentate {
        ObjCFermentate::Empty
    }
}