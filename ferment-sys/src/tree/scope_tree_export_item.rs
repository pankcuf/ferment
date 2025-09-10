use std::cell::RefCell;
use std::fmt::Formatter;
use std::collections::HashSet;
use std::rc::Rc;
use indexmap::IndexMap;
use syn::{Attribute, Item, ItemMod, ItemUse};
use crate::context::{GlobalContext, ScopeChain, ScopeContext, ScopeContextLink};
use crate::ext::MaybeIdent;
use crate::formatter::{format_imported_set, format_tree_exported_dict};
use crate::tree::{ScopeTreeID, GetScopeTreeID};


#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum ScopeTreeExportItem {
    Item(ScopeContextLink, Item),
    Tree(ScopeContextLink, HashSet<ItemUse>, IndexMap<ScopeTreeID, ScopeTreeExportItem>, Vec<Attribute>),
}

impl std::fmt::Debug for ScopeTreeExportItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeTreeExportItem::Item(_, item) =>
                f.write_str(&format!("ScopeTreeExportItem::Item({:?})", item.maybe_ident())),
            ScopeTreeExportItem::Tree(context, imported, exported, attrs) =>
                f.debug_struct("ScopeTreeExportItem::Tree")
                    .field("context", context)
                    .field("imported", &format_imported_set(imported))
                    .field("exported", &format_tree_exported_dict(exported))
                    .field("attrs", attrs)
                    .finish()
        }
    }
}

impl std::fmt::Display for ScopeTreeExportItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl ScopeTreeExportItem {
    pub fn scope(&self) -> ScopeChain {
        match self {
            ScopeTreeExportItem::Item(ctx, ..) => ctx.borrow().scope.clone(),
            ScopeTreeExportItem::Tree(ctx, ..) => ctx.borrow().scope.clone(),
        }
    }
    pub fn tree_with_context_and_exports(context: ScopeContextLink, attrs: &[Attribute]) -> Self {
        Self::Tree(context, HashSet::default(), IndexMap::default(), attrs.to_owned())
    }
    pub fn tree_with_context(scope: &ScopeChain, context: Rc<RefCell<GlobalContext>>, attrs: &[Attribute]) -> Self {
        Self::tree_with_context_and_exports(ScopeContext::cell_with(scope.clone(), context), attrs)
    }
    pub fn item_with_context(scope: &ScopeChain, item: &Item, context: Rc<RefCell<GlobalContext>>) -> Self {
        Self::Item(ScopeContext::cell_with(scope.clone(), context), item.clone())
    }
    pub fn add_item(&mut self, item: Item, scope: ScopeChain) {
        if let ScopeTreeExportItem::Tree(..) = self {
            match &item {
                Item::Use(..) => {},
                Item::Mod(item_mod) => self.add_mod_item(item_mod, &scope),
                Item::Trait(..) |
                Item::Fn(..) |
                Item::Struct(..) |
                Item::Enum(..) |
                Item::Type(..) |
                Item::Impl(..) => self.add_non_mod_item(&item, &scope),
                _ => {}
            };
        }
    }

    fn add_items(&mut self, items: &[Item], scope: &ScopeChain) {
        items.iter().for_each(|item|
            match item {
                Item::Mod(item_mod) =>
                    self.add_mod_item(item_mod, scope),
                Item::Const(_) |
                Item::Enum(_) |
                Item::Fn(_) |
                Item::Impl(_) |
                Item::Struct(_) |
                Item::Trait(_) |
                Item::Type(_) => self.add_non_mod_item(item, scope),
                _ => {}
            }
        );
    }
    fn add_non_mod_item(&mut self, item: &Item, scope: &ScopeChain) {
        if let Self::Tree(scope_context, _, exported, _attrs) = self {
            exported.insert(item.scope_tree_id(), Self::item_with_context(scope, item, scope_context.borrow().context.clone()));
        }
    }

    fn add_mod_item(&mut self, item_mod: &ItemMod, scope: &ScopeChain) {
        let ItemMod { attrs, ident, content, .. } = item_mod;
        let new_export_item = |context: &mut ScopeContextLink| Self::tree_with_context(scope, context.borrow().context.clone(), attrs);
        match content {
            Some((_, items)) => match self {
                Self::Item(context, _) => {
                    let mut inner_tree = new_export_item(context);
                    inner_tree.add_items(items, scope);
                },
                Self::Tree(context, _, exported, _) => {
                    let mut inner_tree = new_export_item(context);
                    inner_tree.add_items(items, scope);
                    exported.insert(ScopeTreeID::from_ident(ident), inner_tree);
                }
            },
            None => if let Self::Tree(context, _, exported, _) = self {
                exported.insert(ScopeTreeID::from_ident(ident), new_export_item(context));
            }
        }
    }
}
