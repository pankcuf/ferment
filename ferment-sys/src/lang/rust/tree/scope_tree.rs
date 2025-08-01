use quote::format_ident;
use syn::{UseRename, UseTree};
use crate::ast::{Depunctuated, SemiPunctuated};
use crate::composable::CfgAttributes;
use crate::composer::{MaybeComposer, SourceAccessible, SourceFermentable};
use crate::lang::RustSpecification;
use crate::presentation::RustFermentate;
use crate::tree::{create_item_use_with_tree, ScopeTree, ScopeTreeItem};

impl SourceFermentable<RustFermentate> for ScopeTree {
    fn ferment(&self) -> RustFermentate {
        let fermentate = Depunctuated::from_iter(self.exported
            .values()
            .filter_map(|item| match item {
                ScopeTreeItem::Item { item, scope, scope_context } =>
                    MaybeComposer::<RustSpecification>::maybe_composer(item, scope, scope_context)
                        .map(|composer| composer.ferment()),
                ScopeTreeItem::Tree { tree } =>
                    Some(tree.ferment())
            }));
        if !fermentate.is_empty() {
            let source = self.source_ref();
            let ctx = source.context.read().unwrap();
            let mut imports = SemiPunctuated::from_iter([
                create_item_use_with_tree(UseTree::Rename(UseRename { ident: format_ident!("crate"), as_token: Default::default(), rename: ctx.config.current_crate.ident() }))
            ]);
            imports.extend(SemiPunctuated::from_iter(self.imported.iter().cloned()));
            RustFermentate::mod_with(self.attrs.cfg_attributes(), self.scope.crate_name(), imports, fermentate)
        } else {
            RustFermentate::Empty
        }
    }
}
