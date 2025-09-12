use quote::format_ident;
use syn::{UseRename, UseTree};
use crate::ast::Depunctuated;
use crate::composable::CfgAttributes;
use crate::composer::{MaybeComposer, SourceAccessible, SourceFermentable};
use crate::ext::PunctuateOne;
use crate::lang::RustSpecification;
use crate::presentation::RustFermentate;
use crate::tree::{create_item_use_with_tree, ScopeTree, ScopeTreeItem};

impl SourceFermentable<RustFermentate> for ScopeTree {
    fn ferment(&self) -> RustFermentate {
        // Ferment children and drop empty results to avoid emitting empty modules.
        let children: Vec<RustFermentate> = self
            .exported
            .values()
            .filter_map(|item| match item {
                ScopeTreeItem::Item { item, scope, scope_context } =>
                    MaybeComposer::<RustSpecification>::maybe_composer(item, scope, scope_context)
                        .map(|composer| composer.ferment()),
                ScopeTreeItem::Tree { tree } => Some(tree.ferment()),
            })
            .filter(|f| !matches!(f, RustFermentate::Empty))
            .collect();

        if children.is_empty() {
            return RustFermentate::Empty;
        }

        let fermentate = Depunctuated::from_iter(children);

        let source = self.source_ref();
        let ctx = source.context.borrow();

        // Only include the alias import (`use crate as <crate_ident>;`) when this module
        // has direct items (non-mod children). If it only nests other modules, skip it.
        let imports = fermentate
            .iter()
            .any(|f| !matches!(f, RustFermentate::Mod { .. }))
            .then(|| create_item_use_with_tree(UseTree::Rename(UseRename { ident: format_ident!("crate"), as_token: Default::default(), rename: ctx.config.current_crate.ident() })).punctuate_one());

        // let mut imports = SemiPunctuated::from_iter(self.imported.iter().cloned());
        // if has_non_mod_child {
        //     imports.insert(0, create_item_use_with_tree(UseTree::Rename(UseRename { ident: format_ident!("crate"), as_token: Default::default(), rename: ctx.config.current_crate.ident() })));
        // }
        RustFermentate::mod_with(self.attrs.cfg_attributes(), self.scope.crate_name(), imports, fermentate)
    }
}
