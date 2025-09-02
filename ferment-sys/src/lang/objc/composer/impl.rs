use crate::composer::{ImplComposer, SourceFermentable};
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};

impl SourceFermentable<ObjCFermentate> for ImplComposer<ObjCSpecification> {
    fn ferment(&self) -> ObjCFermentate {
        ObjCFermentate::Empty
    }
}