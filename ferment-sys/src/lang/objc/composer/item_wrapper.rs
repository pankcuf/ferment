use crate::composer::{ItemComposerWrapper, SourceFermentable};
use crate::lang::objc::ObjCFermentate;

impl<SPEC> ItemComposerWrapper<ObjCFermentate, SPEC>
    where SPEC: crate::lang::objc::ObjCSpecification {
    pub fn ferment(&self) -> ObjCFermentate {
        match self {
            ItemComposerWrapper::Enum(composer) => composer.borrow().ferment(),
            ItemComposerWrapper::EnumVariantNamed(composer) => composer.borrow().composer.borrow().ferment(),
            ItemComposerWrapper::EnumVariantUnnamed(composer) => composer.borrow().composer.borrow().ferment(),
            ItemComposerWrapper::EnumVariantUnit(composer) => composer.borrow().composer.borrow().ferment(),
            ItemComposerWrapper::StructNamed(composer) => composer.borrow().composer.borrow().ferment(),
            ItemComposerWrapper::StructUnnamed(composer) => composer.borrow().composer.borrow().ferment(),
            ItemComposerWrapper::OpaqueStructUnnamed(..) => ObjCFermentate::default(),
            ItemComposerWrapper::OpaqueStructNamed(..) => ObjCFermentate::default(),
            ItemComposerWrapper::Sig(composer) => composer.borrow().ferment(),
            ItemComposerWrapper::TypeAlias(composer) => composer.borrow().composer.borrow().ferment(),
            ItemComposerWrapper::Trait(composer) => composer.borrow().ferment(),
            ItemComposerWrapper::Impl(composer) => composer.borrow().ferment(),
        }
    }
}
