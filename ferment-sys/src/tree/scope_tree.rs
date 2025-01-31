use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, RwLock};
use proc_macro2::Ident;
use quote::{format_ident, ToTokens};
use syn::{Attribute, ItemUse, UseRename, UseTree};
use crate::ast::{Depunctuated, SemiPunctuated};
use crate::composable::CfgAttributes;
use crate::composer::{MaybeComposer, SourceAccessible, SourceFermentable};
use crate::context::{GlobalContext, Scope, ScopeChain, ScopeContext, ScopeContextLink, ScopeInfo};
use crate::conversion::ObjectKind;
use crate::ext::Join;
use crate::formatter::format_tree_item_dict;
use crate::lang::RustSpecification;
use crate::presentation::RustFermentate;
use crate::tree::{ScopeTreeExportID, ScopeTreeExportItem, ScopeTreeItem};

#[derive(Clone)]
pub struct ScopeTree {
    pub attrs: Vec<Attribute>,
    pub scope: ScopeChain,
    pub imported: HashSet<ItemUse>,
    pub exported: HashMap<ScopeTreeExportID, ScopeTreeItem>,
    pub scope_context: ScopeContextLink,
}
impl Debug for ScopeTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("ScopeTree({})", format_tree_item_dict(&self.exported)).as_str())
    }
}

impl ScopeTree {
    pub fn print_scope_tree_with_message(&self, message: &str) {
        self.scope_context.borrow().print_with_message(message)
    }
}

impl SourceAccessible for ScopeTree {
    fn context(&self) -> &ScopeContextLink {
        &self.scope_context
    }
}

impl RustSpecification for ScopeTree {}

impl SourceFermentable<RustFermentate> for ScopeTree {
    fn ferment(&self) -> RustFermentate {
        let source = self.source_ref();
        let fermentate = Depunctuated::from_iter(self.exported
            .values()
            .filter_map(|item| match item {
                ScopeTreeItem::Item { item, scope, scope_context } =>
                    MaybeComposer::<RustFermentate, ScopeTree>::maybe_composer(item, scope, scope_context)
                        .map(|composer| composer.ferment()),
                ScopeTreeItem::Tree { tree } =>
                    Some(tree.ferment())
        }));
        if !fermentate.is_empty() {
            let ctx = source.context.read().unwrap();
            let mut imports = SemiPunctuated::from_iter([
                create_item_use_with_tree(UseTree::Rename(UseRename { ident: format_ident!("crate"), as_token: Default::default(), rename: ctx.config.current_crate.ident() }))
            ]);
            imports.extend(SemiPunctuated::from_iter(self.imported.iter().cloned()));
            let name = if self.scope.is_crate_root() {
                self.scope.crate_ident_ref().to_token_stream()
            } else {
                self.scope.head().to_token_stream()
            };
            RustFermentate::mod_with(self.attrs.cfg_attributes(), name, imports, fermentate)
        } else {
            RustFermentate::Empty
        }
    }
}

pub fn create_generics_scope_tree(root_scope_chain: &ScopeChain, global_context: Arc<RwLock<GlobalContext>>) -> ScopeTree {
    let crate_ident =  root_scope_chain.crate_ident_ref();
    let generics_scope_ident = format_ident!("generics");
    let generics_scope_chain = ScopeChain::Mod {
        info: ScopeInfo {
            attrs: vec![],
            crate_ident: crate_ident.clone(),
            self_scope: Scope::new(root_scope_chain.self_path_holder_ref().joined(&generics_scope_ident), ObjectKind::Empty) },
        parent_scope_chain: root_scope_chain.clone().into() };

    create_scope_tree(
        generics_scope_chain.clone(),
        ScopeContext::cell_with(generics_scope_chain, global_context),
        HashSet::from_iter([
            create_item_use_with_tree(UseTree::Rename(UseRename { ident: format_ident!("crate"), as_token: Default::default(), rename: crate_ident.clone() }))
        ]),
        HashMap::new(),
        vec![]
    )
}

pub fn create_item_use_with_tree(tree: UseTree) -> ItemUse {
    ItemUse {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        use_token: Default::default(),
        leading_colon: None,
        tree,
        semi_token: Default::default(),
    }
}


pub fn create_crate_root_scope_tree(
    crate_ident: Ident,
    scope_context: ScopeContextLink,
    imported: HashSet<ItemUse>,
    exported: HashMap<ScopeTreeExportID, ScopeTreeExportItem>,
    attrs: Vec<Attribute>
) -> ScopeTree {
    // print_phase!("PHASE 2: SCOPE TREE MORPHING", "\n{}", format_tree_exported_dict(&exported));
    create_scope_tree(ScopeChain::crate_root(crate_ident, attrs.clone()), scope_context, imported, exported, attrs)
}

pub fn create_scope_tree(
    scope: ScopeChain,
    scope_context: ScopeContextLink,
    imported: HashSet<ItemUse>,
    exported: HashMap<ScopeTreeExportID, ScopeTreeExportItem>,
    attrs: Vec<Attribute>
) -> ScopeTree {
    let exported = HashMap::from_iter(exported.into_iter()
        .map(|(scope_id, scope_tree_export_item)| {
            let scope_tree_item = match scope_tree_export_item {
                ScopeTreeExportItem::Item(scope_context, item) =>
                    ScopeTreeItem::item(scope.joined(&item), item, scope_context),
                ScopeTreeExportItem::Tree(scope_context, imported, exported, attrs) =>
                    ScopeTreeItem::tree(create_scope_tree(scope_id.create_child_scope(&scope, attrs.clone()), scope_context, imported, exported, attrs))
            };
            (scope_id, scope_tree_item)
        }));
    ScopeTree {
        scope,
        imported,
        attrs,
        scope_context,
        exported,
    }
}