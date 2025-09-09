use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use indexmap::IndexMap;
use proc_macro2::Ident;
use quote::format_ident;
use syn::{Attribute, ItemUse, UseRename, UseTree};
use crate::composer::SourceAccessible;
use crate::context::{GlobalContext, Scope, ScopeContext, ScopeInfo};
use crate::context::{ScopeChain, ScopeContextLink};
use crate::ext::Join;
use crate::formatter::format_tree_item_dict;
use crate::tree::{ScopeTreeID, ScopeTreeItem};
use crate::tree::ScopeTreeExportItem;

#[derive(Clone)]
pub struct ScopeTree {
    pub attrs: Vec<Attribute>,
    pub scope: ScopeChain,
    pub imported: HashSet<ItemUse>,
    pub exported: IndexMap<ScopeTreeID, ScopeTreeItem>,
    pub scope_context: ScopeContextLink,
}
impl Debug for ScopeTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("ScopeTree({})", format_tree_item_dict(&self.exported)))
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


#[allow(unused)]
pub fn create_generics_scope_tree(root_scope_chain: &ScopeChain, global_context: Rc<RefCell<GlobalContext>>) -> ScopeTree {
    let rename =  root_scope_chain.crate_ident();
    let generics_scope_ident = format_ident!("generics");
    let generics_scope_chain = ScopeChain::r#mod(
        ScopeInfo::attr_less(rename.clone(), Scope::empty(root_scope_chain.self_path_ref().joined(&generics_scope_ident))),
        root_scope_chain.clone());

    create_scope_tree(
        generics_scope_chain.clone(),
        ScopeContext::cell_with(generics_scope_chain, global_context),
        HashSet::from_iter([
            create_item_use_with_tree(UseTree::Rename(UseRename { ident: format_ident!("crate"), as_token: Default::default(), rename }))
        ]),
        IndexMap::new(),
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


#[allow(unused)]
pub fn create_crate_root_scope_tree(
    crate_ident: Ident,
    scope_context: ScopeContextLink,
    imported: HashSet<ItemUse>,
    exported: IndexMap<ScopeTreeID, ScopeTreeExportItem>,
    attrs: Vec<Attribute>
) -> ScopeTree {
    // print_phase!("PHASE 2: SCOPE TREE MORPHING", "\n{}", format_tree_exported_dict(&exported));
    create_scope_tree(ScopeChain::crate_root(crate_ident, attrs.clone()), scope_context, imported, exported, attrs)
}

#[allow(unused)]
pub fn create_scope_tree(
    scope: ScopeChain,
    scope_context: ScopeContextLink,
    imported: HashSet<ItemUse>,
    exported: IndexMap<ScopeTreeID, ScopeTreeExportItem>,
    attrs: Vec<Attribute>
) -> ScopeTree {
    let exported = IndexMap::from_iter(exported.into_iter()
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