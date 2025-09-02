use crate::composer::{ItemComposerWrapper, SourceFermentable};
use crate::lang::RustSpecification;
use crate::presentation::RustFermentate;

impl ItemComposerWrapper<RustSpecification> {
    pub fn ferment(&self) -> RustFermentate {
        match self {
            ItemComposerWrapper::Enum(composer) => composer.borrow().ferment(),
            ItemComposerWrapper::EnumVariantNamed(composer) => composer.borrow().composer.borrow().ferment(),
            ItemComposerWrapper::EnumVariantUnnamed(composer) => composer.borrow().composer.borrow().ferment(),
            ItemComposerWrapper::EnumVariantUnit(composer) => composer.borrow().composer.borrow().ferment(),
            ItemComposerWrapper::StructNamed(composer) => composer.borrow().composer.borrow().ferment(),
            ItemComposerWrapper::StructUnnamed(composer) => composer.borrow().composer.borrow().ferment(),
            ItemComposerWrapper::OpaqueStructUnnamed(composer) => composer.borrow().composer.borrow().ferment(),
            ItemComposerWrapper::OpaqueStructNamed(composer) => composer.borrow().composer.borrow().ferment(),
            ItemComposerWrapper::Sig(composer) => composer.borrow().ferment(),
            ItemComposerWrapper::TypeAlias(composer) => composer.borrow().composer.borrow().ferment(),
            ItemComposerWrapper::Trait(composer) => composer.borrow().ferment(),
            ItemComposerWrapper::Impl(composer) => composer.borrow().ferment(),
        }
    }
}

