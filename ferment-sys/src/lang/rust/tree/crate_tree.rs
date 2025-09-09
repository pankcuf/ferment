use crate::ast::{Depunctuated, SemiPunctuated};
use crate::composer::{GenericComposer, SourceAccessible, SourceComposable, SourceFermentable};
use crate::lang::RustSpecification;
use crate::presentation::RustFermentate;
use crate::tree::{CrateTree, ScopeTree};

impl SourceFermentable<RustFermentate> for CrateTree {
    fn ferment(&self) -> RustFermentate {
        let Self { attrs, crates, generics_tree: ScopeTree { imported, .. }} = self;
        let source = self.source_ref();
        let reg_conversions = Depunctuated::from_iter(crates.iter().map(SourceFermentable::<RustFermentate>::ferment));
        let generic_imports = SemiPunctuated::from_iter(imported.iter().cloned());
        let generic_conversions = Depunctuated::from_iter(
            source.context
                .borrow()
                .refined_mixins
                .iter()
                .filter_map(|mixin_context| GenericComposer::<RustSpecification>::mixin(mixin_context, self.context()))
                .flat_map(|composer| composer.borrow().compose(&source)));

        RustFermentate::Root {
            mods: Depunctuated::from_iter([
                RustFermentate::types(attrs, reg_conversions),
                RustFermentate::generics(attrs, generic_imports, generic_conversions)
            ])
        }
    }
}
