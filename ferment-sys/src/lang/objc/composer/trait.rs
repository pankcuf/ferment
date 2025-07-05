use crate::composer::{SourceFermentable, TraitComposer};
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};

impl SourceFermentable<ObjCFermentate> for TraitComposer<ObjCSpecification> {
    fn ferment(&self) -> ObjCFermentate {
        ObjCFermentate::Empty
    }
}