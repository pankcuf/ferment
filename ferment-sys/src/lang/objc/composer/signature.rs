use crate::composer::{SigComposer, SourceFermentable};
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};

impl SourceFermentable<ObjCFermentate> for SigComposer<ObjCSpecification> {
    fn ferment(&self) -> ObjCFermentate {
        ObjCFermentate::Empty
    }
}