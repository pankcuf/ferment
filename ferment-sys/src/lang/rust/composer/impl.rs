use crate::ast::Depunctuated;
use crate::composer::{ImplComposer, SourceAccessible, SourceComposable, SourceFermentable};
use crate::lang::RustSpecification;
use crate::presentation::{DocPresentation, RustFermentate};

impl SourceFermentable<RustFermentate> for ImplComposer<RustSpecification> {
    fn ferment(&self) -> RustFermentate {
        let mut items = Depunctuated::<RustFermentate>::new();
        self.methods.iter().for_each(|sig_composer| {
            let fermentate = sig_composer.borrow().ferment();
            items.push(fermentate);
        });
        let vtable = self.vtable.as_ref()
            .map(|composer| {
                let composer = composer.borrow();
                let composer_source = composer.source_ref();
                composer.compose(&composer_source)
            });
        RustFermentate::Impl { comment: DocPresentation::Empty, items, vtable }
    }
}

