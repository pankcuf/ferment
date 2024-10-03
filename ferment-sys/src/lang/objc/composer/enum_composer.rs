use crate::composer::{EnumComposer, SourceFermentable};

impl<SPEC> SourceFermentable<crate::lang::objc::ObjCFermentate> for EnumComposer<crate::lang::objc::ObjCFermentate, SPEC>
    where SPEC: crate::lang::objc::ObjCSpecification {
    fn ferment(&self) -> crate::lang::objc::ObjCFermentate {
        crate::lang::objc::ObjCFermentate::Empty
    }
}




