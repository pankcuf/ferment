use crate::composer::FieldComposers;
use crate::lang::objc::composers::AttrWrapper;
use crate::lang::objc::ObjCFermentate;

pub struct ArgsComposer {
    pub fields: FieldComposers<ObjCFermentate, AttrWrapper>
}
